use std::{cell::RefCell, rc::Rc};

use result_or_err::ResultOrErr;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDivElement, HtmlElement, PointerEvent};

use crate::{
	components::Label,
	elements::{div, hr, on, styled},
	events::SharedEventListener,
	Component, ComponentContent, DynamicComponent,
};

use super::Button;

pub struct SideBar {
	parent: RefCell<Element>,
	element: HtmlDivElement,
	content_holder: HtmlDivElement,

	x_button: Component<Button>,
	title: Component<Label>,
	content: RefCell<Option<Box<dyn DynamicComponent>>>,

	resize_down_listener: SharedEventListener<PointerEvent>,
	resize_move_listener: SharedEventListener<PointerEvent>,
	resize_up_listener: SharedEventListener<PointerEvent>,
}

impl SideBar {
	pub fn new(
		parent: Element,
		sidebar_css: &str,
		handle_css: &str,
		internal_css: &str,
		controls_css: &str,
		button_css: &str,
		title_css: &str,
		content_holder_css: &str,
	) -> Rc<Self> {
		let element = styled(div(), sidebar_css);

		let resize_handle = on(&element, styled(div(), handle_css));
		let internal = on(&element, styled(div(), internal_css));
		let controls = on(&internal, styled(div(), controls_css));
		let content_holder = on(&internal, styled(div(), content_holder_css));

		let resize_down_listener =
			SharedEventListener::<PointerEvent>::new(resize_handle.clone().into(), "pointerdown");
		let resize_move_listener =
			SharedEventListener::<PointerEvent>::new(resize_handle.clone().into(), "pointermove");
		let resize_up_listener =
			SharedEventListener::<PointerEvent>::new(resize_handle.clone().into(), "pointerup");

		let x_button = Component::make(Button::new(None, button_css));
		x_button.mount_in(&controls).or_err(()).unwrap();
		let title = Component::make(Label::new("", title_css));
		title.mount_in(&controls);

		let this = Rc::new(Self {
			parent: RefCell::new(parent),
			element,
			content_holder,

			x_button,
			title,
			content: RefCell::new(None),

			resize_down_listener,
			resize_move_listener,
			resize_up_listener,
		});

		this.resize_down_listener.set_handler({
			let this = this.clone();
			move |event: PointerEvent| {
				event.prevent_default();
				event.stop_propagation();

				if event.buttons() != 1 {
					return;
				};

				event.target().unwrap().dyn_into::<HtmlElement>().unwrap().set_pointer_capture(event.pointer_id());

				let width = this.element.client_width();
				let start = event.client_x();

				this.resize_move_listener.set_handler({
					let this = this.clone();
					let width = width.clone();
					let start = start.clone();
					move |event: PointerEvent| {
						event.prevent_default();
						event.stop_propagation();

						let diff = event.client_x() - start;
						this.element.style().set_property("width", format!("{}px", width - diff).as_str());
					}
				});

				this.resize_up_listener.set_handler({
					let this = this.clone();
					move |event: PointerEvent| {
						event.prevent_default();
						event.stop_propagation();

						this.resize_move_listener.remove_handler();
						this.resize_up_listener.remove_handler();
					}
				});
			}
		});

		this.x_button.on_click.set_handler({
			let this = this.clone();
			move |_| {
				this.close();
			}
		});

		this
	}

	pub fn open(&self, content: impl ComponentContent + 'static) {
		let Ok(parent) = self.parent.try_borrow() else { return };
		let Ok(mut c) = self.content.try_borrow_mut() else { return };

		let component = Component::make(content);
		component.mount_in(&self.content_holder);
		*c = Some(Box::new(component));

		parent.append_child(&self.element);
	}

	pub fn close(&self) {
		let Ok(mut content) = self.content.try_borrow_mut() else { return };
		*content = None;
		self.element.remove();
	}

	pub fn set_title(&self, text: Option<&str>) {
		self.title.set_text(text);
	}
}

impl ComponentContent for SideBar {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
	fn mount_in(&self, element: &Element) -> anyhow::Result<()> {
		let mut parent = self.parent.try_borrow_mut()?;
		*parent = element.clone();
		Ok(())
	}
}
