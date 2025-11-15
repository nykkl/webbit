use std::rc::Rc;

use anyhow::Result;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, HtmlCanvasElement, ResizeObserver};

use crate::{elements::*, events::CustomEventListener, ComponentContent};

pub struct ResizeCanvas {
	pub canvas: HtmlCanvasElement,
	pub on_resize: CustomEventListener<(u32, u32)>,
}

impl ResizeCanvas {
	pub fn new(css: &'static str) -> Rc<Self> {
		let canvas = styled(canvas(), css);

		let this = Rc::new(Self { canvas, on_resize: CustomEventListener::new() });

		let on_resize = {
			let this = this.clone();
			move || {
				let (w, h) = this.size();
				this.set_context_size(w, h);
				this.on_resize.fire((w, h));
			}
		};

		let callback = Closure::<dyn FnMut()>::new(on_resize);
		let resizer = ResizeObserver::new(callback.into_js_value().unchecked_ref()).unwrap();
		resizer.observe(this.element());

		this
	}

	pub fn size(&self) -> (u32, u32) {
		(self.canvas.client_width() as u32, self.canvas.client_height() as u32)
	}

	fn set_context_size(&self, w: u32, h: u32) {
		self.canvas.set_width(w);
		self.canvas.set_height(h);
	}
}

impl ComponentContent for ResizeCanvas {
	fn element(&self) -> &Element {
		&self.canvas
	}

	fn update(&self) -> Result<()> {
		let (w, h) = self.size();
		self.set_context_size(w, h);

		Ok(())
	}

	fn destroy(&self) -> Result<()> {
		self.on_resize.remove_handler();
		Ok(())
	}
}
