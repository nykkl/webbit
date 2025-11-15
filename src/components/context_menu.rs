use std::rc::Rc;

use anyhow::Result;
use web_sys::{HtmlDivElement, PointerEvent};

use crate::{components::Button, elements::*, events::BubbleStopper, Component, ComponentContent};

pub struct ContextMenu {
	element: HtmlDivElement,

	quick_actions: Vec<Component<Button>>,
	actions: Vec<Component<Button>>,
}

pub type MenuAction = (&'static str, Box<dyn FnMut(PointerEvent)>);

impl ContextMenu {
	pub fn new(quick_actions: Vec<MenuAction>, actions: Vec<MenuAction>, css: &'static str) -> Self {
		let element = styled(div(), ["context-menu", css].join(" ").as_str());
		BubbleStopper::new(element.clone().into(), "click");
		BubbleStopper::new(element.clone().into(), "pointerdown");
		BubbleStopper::new(element.clone().into(), "pointermove");
		BubbleStopper::new(element.clone().into(), "pointerup");
		BubbleStopper::new(element.clone().into(), "contextmenu");

		let quick_action_div = on(&element, styled(div(), "context-menu-quick-section"));
		let action_div = on(&element, styled(div(), "context-menu-section"));

		let quick_actions = quick_actions
			.into_iter()
			.map(|(name, action)| {
				let component =
					Component::make(Button::new_with_handler(Some(name), "context-menu-quick-button", action));
				component.mount_in(&quick_action_div);
				component
			})
			.collect();
		let actions = actions
			.into_iter()
			.map(|(name, action)| {
				let component = Component::make(Button::new_with_handler(Some(name), "context-menu-button", action));
				component.mount_in(&action_div);
				component
			})
			.collect();

		Self { element, quick_actions, actions }
	}
}

impl ComponentContent for ContextMenu {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
}
