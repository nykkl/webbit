use std::rc::Rc;

use anyhow::Result;
use web_sys::{HtmlDivElement, PointerEvent};

use crate::{
	elements::*,
	events::{BubbleStopper, SharedEventListener},
	ComponentContent,
};

pub struct Button {
	element: HtmlDivElement,

	pub on_click: SharedEventListener<PointerEvent>,
}
impl Button {
	pub fn new(name: Option<&str>, css: &str) -> Self {
		let element = styled(div(), css);
		BubbleStopper::new(element.clone().into(), "click");
		BubbleStopper::new(element.clone().into(), "pointerdown");
		BubbleStopper::new(element.clone().into(), "pointermove");
		BubbleStopper::new(element.clone().into(), "pointerup");
		BubbleStopper::new(element.clone().into(), "contextmenu");
		if let Some(name) = name {
			element.set_text_content(Some(name));
		}

		let on_click = SharedEventListener::<PointerEvent>::new(element.clone().into(), "click");

		Self { element, on_click }
	}

	pub fn new_with_handler(
		name: Option<&str>,
		css: &str,
		on_click: impl FnMut(PointerEvent) + 'static,
	) -> Self {
		let this = Self::new(name, css);
		this.on_click.set_handler(on_click);
		this
	}

	pub fn set_text(&self, text: &str) {
		self.element.set_text_content(Some(text));
	}
}
impl ComponentContent for Button {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
	fn destroy(&self) -> Result<()> {
		self.on_click.remove_handler();
		Ok(())
	}
}
