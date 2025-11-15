use std::rc::Rc;

use anyhow::Result;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDivElement, HtmlInputElement};

use crate::{
	elements::*,
	events::{BubbleStopper, CustomEventListener, SharedEventListener},
	ComponentContent,
};

pub struct Checkbox {
	element: HtmlDivElement,

	pub on_change: CustomEventListener<bool>,

	on_click: SharedEventListener<Event>,

	css: &'static str,
}
impl Checkbox {
	pub fn new(name: Option<&str>, value: bool, css: &'static str) -> Rc<Self> {
		let element = styled(div(), css);
		BubbleStopper::new(element.clone().into(), "click");
		BubbleStopper::new(element.clone().into(), "pointerdown");
		BubbleStopper::new(element.clone().into(), "pointermove");
		BubbleStopper::new(element.clone().into(), "pointerup");
		BubbleStopper::new(element.clone().into(), "contextmenu");

		// checkbox
		let checkbox = on(&element, checkbox(value));
		// label
		if let Some(name) = name {
			on(&element, label(name));
		}

		let this = Rc::new(Self {
			element,

			on_change: CustomEventListener::new(),
			on_click: SharedEventListener::<Event>::new(checkbox.clone().into(), "change"),
			css,
		});

		this.on_click.set_handler({
			let this = this.clone();
			move |event| {
				let target = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
				this.on_change.fire(target.checked());
			}
		});

		this
	}
}
impl ComponentContent for Checkbox {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
	fn destroy(&self) -> Result<()> {
		self.on_click.remove_handler();
		Ok(())
	}
}
