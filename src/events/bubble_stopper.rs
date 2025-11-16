use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, EventTarget};

/// Stops the specified event from bubbling up.
/// By default it will also prvent the default action.
/// By default this does not expire. That means you can drop it and it will keep working.
///
/// These defaults can be changed by configuring the object with [BubbleStopper::configure].
pub struct BubbleStopper {
	closure: Option<Closure<dyn FnMut(Event)>>,
	expires: bool,
}
impl BubbleStopper {
	pub fn new(target: EventTarget, eventname: &'static str) -> Self {
		Self::make(target, eventname, BubbleStopperConfig::default())
	}
	pub fn configure(
		target: EventTarget,
		eventname: &'static str,
		configure: impl FnOnce(BubbleStopperConfig) -> BubbleStopperConfig,
	) -> Self {
		let config = configure(BubbleStopperConfig::default());
		Self::make(target, eventname, config)
	}
	fn make(target: EventTarget, eventname: &'static str, config: BubbleStopperConfig) -> Self {
		let handler = match config.prevent_default {
			false => |e: Event| e.stop_propagation(),
			true => |e: Event| {
				e.prevent_default();
				e.stop_propagation()
			},
		};
		let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(Event)>);
		target
			.add_event_listener_with_callback(eventname, closure.as_ref().unchecked_ref())
			.expect("Failed to add event handler to event listener.");

		Self { closure: Some(closure), expires: config.expires }
	}

	/// Resume event bubbling when [BubbleStopper] is dropped.
	/// You will now have to keep you [BubbleStopper] object alive for as long as you want to prevent event bubbling.
	pub fn expiring(mut self) -> Self {
		self.expires = true;
		self
	}
	/// Drops the object but keeps it from expiring.
	/// That means it will keep preventing event bubbling like it was still alive.
	pub fn forget(mut self) {
		self.forget_closure();
	}
	fn forget_closure(&mut self) {
		if let Some(closure) = self.closure.take() {
			closure.forget();
		}
	}
}
impl Drop for BubbleStopper {
	fn drop(&mut self) {
		if !self.expires {
			self.forget_closure();
		}
	}
}

/// Configuration for [BubbleStopper].
/// Calling a method on this will change the configuration as described.
pub struct BubbleStopperConfig {
	prevent_default: bool,
	expires: bool,
}
impl Default for BubbleStopperConfig {
	fn default() -> Self {
		Self { prevent_default: true, expires: false }
	}
}
impl BubbleStopperConfig {
	/// Don't prevent default action.
	pub fn permit_default(mut self) -> Self {
		self.prevent_default = false;
		self
	}
	/// Resume event bubbling when [BubbleStopper] is dropped.
	/// You will now have to keep you [BubbleStopper] object alive for as long as you want to prevent event bubbling.
	pub fn expiring(mut self) -> Self {
		self.expires = true;
		self
	}
}
