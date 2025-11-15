use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, EventTarget};

/// Stops the specified event from bubbling up.
///
/// This is a dummy struct. It doesn't do anything after its instantiation.
/// You can just call `new()` and then drop it.
/// This is only done for the sake of similarity with the other event listeners.
pub struct BubbleStopper;

impl BubbleStopper {
	pub fn new(target: EventTarget, eventname: &'static str) -> Self {
		let handler = |e: Event| {
			e.stop_propagation();
		};
		let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(Event)>);
		target
			.add_event_listener_with_callback(eventname, closure.as_ref().unchecked_ref())
			.expect("Failed to add event handler to event listener.");
		closure.forget();

		Self {}
	}
}
