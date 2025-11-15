use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};
use web_sys::EventTarget;

/// Handles events from a target.
///
/// # Type parameters:
/// - `E`: The type of the event argument.
///
/// # Remarks:
/// - Simplifies event handling.
/// - Could be used wrapped in an `Rc<RefCell<>>` to pass it into the handler and manipulate event handling inside the handler.
/// - You need to keep this object around as long as you want the event handler to be active or call `forget()` on it.
/// - Once you called `forget()`, you can't remove the handler anymore.
///
/// This is meant to be a setup helper. I don't know how performant it is, so maybe don't use it in a critical path.
pub struct EventListener<E: FromWasmAbi + 'static> {
	target: EventTarget,
	eventname: &'static str,
	handler: Option<Closure<dyn FnMut(E)>>,
}

impl<E: FromWasmAbi> EventListener<E> {
	pub fn new(target: EventTarget, eventname: &'static str) -> Self {
		Self { target, eventname, handler: None }
	}

	/// Use the given event handler to handle events from the target.
	///
	/// Removes the old handler.
	/// Events will henceforth be handled only by the given `handler`.
	pub fn set_handler(&mut self, handler: impl FnMut(E) + 'static) {
		self.remove_handler();

		let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(E)>);
		self
			.target
			.add_event_listener_with_callback(self.eventname, closure.as_ref().unchecked_ref())
			.expect("Failed to add event handler to event listener.");
		self.handler = Some(closure);
	}

	/// Does the same thing as `set_handler()` just with a different function signature and returns the same handler it is called on.
	///
	/// This is just syntactic sugar, that allows for something like: `EventListener::new().with_handler().forget()`.
	pub fn with_handler(mut self, handler: impl FnMut(E) + 'static) -> Self {
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
	pub fn remove_handler(&mut self) {
		let Some(handler) = self.handler.take() else { return };
		self
			.target
			.remove_event_listener_with_callback(self.eventname, handler.as_ref().unchecked_ref())
			.expect("Failed to remove event handler from event listener.");
	}

	/// Drops this object without removing the handler.
	///
	/// This means events will continue to be handled by the current handler.
	pub fn forget(mut self) {
		let Some(handler) = self.handler.take() else { return };
		handler.forget();
	}
}

impl<E: FromWasmAbi> Drop for EventListener<E> {
	fn drop(&mut self) {
		self.remove_handler();
	}
}
