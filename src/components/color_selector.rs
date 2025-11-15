use std::rc::Rc;

use anyhow::Result;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlInputElement, InputEvent};

use crate::{
	elements::*,
	events::{BubbleStopper, CustomEventListener, SharedEventListener},
	ComponentContent,
};

pub struct ColorSelector {
	element: HtmlDivElement,
	color: HtmlInputElement,
	text: HtmlInputElement,

	pub on_change: CustomEventListener<String>,

	color_change: SharedEventListener<InputEvent>,
	text_change: SharedEventListener<InputEvent>,
}
impl ColorSelector {
	pub fn new(name: Option<&str>, value: &str, css: &str) -> Rc<Self> {
		let element = styled(div(), css);
		BubbleStopper::new(element.clone().into(), "click");
		BubbleStopper::new(element.clone().into(), "pointerdown");
		BubbleStopper::new(element.clone().into(), "pointermove");
		BubbleStopper::new(element.clone().into(), "pointerup");
		BubbleStopper::new(element.clone().into(), "contextmenu");

		// label
		if let Some(name) = name {
			on(&element, label(name));
		}
		// color
		let color = on(&element, color(value));
		let color_change = SharedEventListener::<InputEvent>::new(color.clone().into(), "input");
		// text
		let text = on(&element, text(value));
		let text_change = SharedEventListener::<InputEvent>::new(text.clone().into(), "input");

		let this =
			Rc::new(Self { element, color, text, on_change: CustomEventListener::new(), color_change, text_change });

		this.color_change.set_handler({
			let this = this.clone();
			move |event| {
				let target = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
				this.text.set_value(&target.value());
				this.on_change.fire(target.value());
			}
		});
		this.text_change.set_handler({
			let this = this.clone();
			move |event| {
				let target = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
				this.color.set_value(&target.value());
				this.on_change.fire(target.value());
			}
		});

		this
	}

	/// Set the value displayed by the ui component.
	///
	/// Does not trigger `on_change`!
	pub fn set_value(&self, value: String) {
		self.text.set_value(&value);
		self.color.set_value(&value);
	}
}
impl ComponentContent for ColorSelector {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
	fn destroy(&self) -> Result<()> {
		self.color_change.remove_handler();
		Ok(())
	}
}
