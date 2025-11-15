use std::cell::{RefCell, RefMut};

use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};
use web_sys::EventTarget;

/// Like `EventListener`, but to be able to share it, all methods are immutable.
/// To be able to use this in multiple location (e.g. using `Rc<>`) we use interior mutability (runtime borrow-checking).
pub struct SharedEventListener<E: FromWasmAbi + 'static> {
	target: EventTarget,
	eventname: &'static str,
	handler: RefCell<Option<Closure<dyn FnMut(E)>>>, // we only borrow once per function and don't expose this RefCell so borrow() should always work
}

impl<E: FromWasmAbi> SharedEventListener<E> {
	pub fn new(target: EventTarget, eventname: &'static str) -> Self {
		Self { target, eventname, handler: RefCell::new(None) }
	}

	/// Use the given event handler to handle events from the target.
	///
	/// Removes the old handler.
	/// Events will henceforth be handled only by the given `handler`.
	pub fn set_handler(&self, handler: impl FnMut(E) + 'static) {
		// SAFETY: handler is never allowed to leave this struct
		// and in this struct care is taken to never borrow handler twice
		let mut h = self.handler.borrow_mut();

		Self::deregister_handler(&self.target, self.eventname, &mut h);

		let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(E)>);
		self
			.target
			.add_event_listener_with_callback(self.eventname, closure.as_ref().unchecked_ref())
			.expect("Failed to add event handler to event listener.");
		*h = Some(closure);
	}

	/// Does the same thing as `set_handler()` just with a different function signature and returns the same handler it is called on.
	///
	/// This is just syntactic sugar, that allows for something like: `DynamicEventListener::new().with_handler().unwrap().forget()`.
	///
	/// Returns self on success and (self, handler) on error.
	pub fn with_handler(self, handler: impl FnMut(E) + 'static) -> Self {
		self.set_handler(handler);
		self
	}

	/// Removes the current event handler.
	///
	/// This means events will no longer be handled.
	///
	/// You proably don't need this method. Calling `set_handler` or dropping the listener will remove the current handler automatically:
	/// - So if you want to stop handling events altogether, just drop the listener.
	/// - If you want to change the handler, just call `set_handler` directly.
	pub fn remove_handler(&self) {
		// SAFETY: handler is never allowed to leave this struct
		// and in this struct care is taken to never borrow handler twice
		let mut h = self.handler.borrow_mut();
		Self::deregister_handler(&self.target, self.eventname, &mut h);
	}

	/// Drops this object without removing the handler.
	///
	/// This means events will continue to be handled by the current handler.
	pub fn forget(self) {
		// SAFETY: handler is never allowed to leave this struct
		// and in this struct care is taken to never borrow handler twice
		let mut h = self.handler.borrow_mut();
		if let Some(handler) = h.take() {
			handler.forget();
		}
	}

	fn deregister_handler(
		target: &EventTarget,
		eventname: &str,
		handler: &mut RefMut<Option<Closure<dyn FnMut(E)>>>,
	) {
		let Some(handler) = handler.take() else { return };
		target
			.remove_event_listener_with_callback(eventname, handler.as_ref().unchecked_ref())
			.expect("Failed to remove event handler from event listener.");
	}
}

impl<E: FromWasmAbi> Drop for SharedEventListener<E> {
	fn drop(&mut self) {
		self.remove_handler();
	}
}
