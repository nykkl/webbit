use std::{
	ops::{Add, Sub},
	rc::Rc,
	str::FromStr,
};

use anyhow::Result;
use result_or_err::ResultOrErr;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlInputElement, InputEvent};

use crate::{
	elements::*,
	events::{BubbleStopper, CustomEventListener, SharedEventListener},
	Component, ComponentContent,
};

use super::Button;

pub struct Slider<T> {
	element: HtmlDivElement,
	slider: HtmlInputElement,
	buttons: Option<(Component<Button>, Component<Button>)>,
	text: HtmlInputElement,

	pub on_change: CustomEventListener<T>,

	slider_change: SharedEventListener<InputEvent>,
	text_change: SharedEventListener<InputEvent>,
}
impl<T: FromStr + ToString + Clone + Add<T, Output = T> + Sub<T, Output = T> + 'static> Slider<T> {
	pub fn new(name: Option<&str>, value: T, min: T, max: T, step: T, buttons: bool, css: &str) -> Rc<Self> {
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
		// slider
		let slider = on(&element, slider(&value, &min, &max, &step));
		let slider_change = SharedEventListener::<InputEvent>::new(slider.clone().into(), "input");
		// text
		let text = on(&element, text(&value.to_string()));
		text.set_min(&min.to_string());
		text.set_max(&max.to_string());
		let text_change = SharedEventListener::<InputEvent>::new(text.clone().into(), "input");
		// buttons
		let buttons = match buttons {
			true => {
				let div = on(&element, div());

				let minus = Component::make(Button::new(Some("-"), "settingsButton"));
				minus.mount_in(&div).unwrap();
				let plus = Component::make(Button::new(Some("+"), "settingsButton"));
				plus.mount_in(&div).unwrap();

				Some((minus, plus))
			},
			false => None,
		};

		let this = Rc::new(Self {
			element,
			slider,
			buttons,
			text,

			on_change: CustomEventListener::new(),

			slider_change,
			text_change,
		});

		this.slider_change.set_handler({
			let this = this.clone();
			move |event| {
				let target = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
				this.text.set_value(&target.value());
				if let Ok(value) = target.value().parse::<T>() {
					this.on_change.fire(value);
				}
			}
		});
		this.text_change.set_handler({
			let this = this.clone();
			move |event| {
				let target = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
				this.slider.set_value(&target.value());
				if let Ok(value) = target.value().parse::<T>() {
					this.on_change.fire(value);
				}
			}
		});
		if let Some((minus, plus)) = &this.buttons {
			minus.on_click.set_handler({
				let this = this.clone();
				let step = step.clone();
				move |_| {
					this.change_value(this.get_value() - step.clone());
				}
			});
			plus.on_click.set_handler({
				let this = this.clone();
				move |_| {
					this.change_value(this.get_value() + step.clone());
				}
			});
		}

		this
	}

	/// Set the value displayed by the ui component.
	///
	/// Does not trigger `on_change`!
	pub fn set_value(&self, value: T) {
		self.text.set_value(&value.to_string());
		self.slider.set_value(&value.to_string());
	}

	pub fn get_value(&self) -> T {
		self.text.value().parse::<T>().or_err(()).unwrap()
	}

	/// Set the value displayed by the ui component and trigger `on_change`.
	///
	/// Triggers `on_change` immediately. Use carefully to not create infinite loops, ...!
	pub fn change_value(&self, value: T) {
		self.set_value(value.clone());
		self.on_change.fire(value);
	}
}
impl<T: FromStr + ToString> ComponentContent for Slider<T> {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
	fn destroy(&self) -> Result<()> {
		self.slider_change.remove_handler();
		Ok(())
	}
}
