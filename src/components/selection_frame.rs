use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use ncollide2d::na::{convert, Affine2, Scale2, Translation2};
use result_or_err::ResultOrErr;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlDivElement, HtmlElement, PointerEvent};

use crate::{
	common::{Bounds, Number, Vector},
	elements::*,
	events::{CustomEventListener, SharedEventListener},
	ComponentContent,
};

pub struct SelectionFrame {
	/// a function that defines the selections bounds
	get_bounds: Box<dyn Fn() -> Option<Bounds>>,
	/// a function that lets the user render stuff to the selection
	integrate_transformation: Box<dyn Fn(Affine2<Number>, &Self) -> Result<(), ()>>,
	pub on_click: CustomEventListener<PointerEvent>,
	pub on_context: CustomEventListener<PointerEvent>,
	pub outline: HtmlDivElement,
	pub control_knob: HtmlDivElement,

	// State
	bounds: RefCell<Option<Bounds>>,
	translation: RefCell<Vector>,
	scale: RefCell<Number>,
	transformation_locked: RefCell<bool>, // to prevent overlapping translate and scale actions
	integrate_on_move: RefCell<bool>,
	pub on_render: CustomEventListener<()>,

	// UI
	parent: HtmlElement,
	element: HtmlDivElement,
	pub canvas: HtmlCanvasElement,
	move_button: HtmlDivElement,
	resize_button: HtmlDivElement,
	move_down_listener: SharedEventListener<PointerEvent>,
	move_move_listener: SharedEventListener<PointerEvent>,
	move_up_listener: SharedEventListener<PointerEvent>,
	click_listener: SharedEventListener<PointerEvent>,
	context_listener: SharedEventListener<PointerEvent>,
	resize_down_listener: SharedEventListener<PointerEvent>,
	resize_move_listener: SharedEventListener<PointerEvent>,
	resize_up_listener: SharedEventListener<PointerEvent>,
}

impl SelectionFrame {
	pub fn new(
		parent: HtmlElement,
		compute_bonds: impl Fn() -> Option<Bounds> + 'static,
		integrate_transformation: impl Fn(Affine2<Number>, &Self) -> Result<(), ()> + 'static,
	) -> Rc<Self> {
		let element = styled(div(), "selection-frame");

		let canvas = on(&element, styled(canvas(), "selection-canvas"));
		let outline = on(&element, styled(div(), "selection-outline"));

		let border = on(&outline, styled(div(), "selection-border"));

		let resize_button = on(&outline, styled(div(), "resize-button"));
		let resize_down_listener =
			SharedEventListener::<PointerEvent>::new(resize_button.clone().into(), "pointerdown");
		let resize_move_listener =
			SharedEventListener::<PointerEvent>::new(resize_button.clone().into(), "pointermove");
		let resize_up_listener =
			SharedEventListener::<PointerEvent>::new(resize_button.clone().into(), "pointerup");

		let control_knob = on(&outline, styled(div(), "control-knob"));
		let move_button = on(&control_knob, styled(div(), "move-button"));
		let move_down_listener =
			SharedEventListener::<PointerEvent>::new(move_button.clone().into(), "pointerdown");
		let move_move_listener =
			SharedEventListener::<PointerEvent>::new(move_button.clone().into(), "pointermove");
		let move_up_listener = SharedEventListener::<PointerEvent>::new(move_button.clone().into(), "pointerup");
		let click_listener = SharedEventListener::<PointerEvent>::new(move_button.clone().into(), "click");
		let context_listener = SharedEventListener::<PointerEvent>::new(move_button.clone().into(), "contextmenu");
		let on_click = CustomEventListener::<PointerEvent>::new();
		let on_context = CustomEventListener::<PointerEvent>::new();

		let this = Rc::new(Self {
			get_bounds: Box::new(compute_bonds),
			integrate_transformation: Box::new(integrate_transformation),
			outline,
			control_knob,
			on_click,
			on_context,

			translation: RefCell::new(Vector::zero()),
			scale: RefCell::new(1.0),
			transformation_locked: RefCell::new(false),
			integrate_on_move: RefCell::new(false),
			on_render: CustomEventListener::new(),

			bounds: RefCell::new(None),

			parent,
			element,
			canvas,
			move_button,
			resize_button,

			move_down_listener,
			move_move_listener,
			move_up_listener,
			click_listener,
			context_listener,
			resize_down_listener,
			resize_move_listener,
			resize_up_listener,
		});

		this.move_down_listener.set_handler({
			let this = this.clone();
			move |event: PointerEvent| {
				event.stop_propagation();

				if event.buttons() != 1 {
					return;
				};
				event.prevent_default();

				if !this.lock_transformation() {
					return;
				};
				event.target().unwrap().dyn_into::<HtmlElement>().unwrap().set_pointer_capture(event.pointer_id());
				let start = this.capture_position(&event).unwrap();

				this.move_move_listener.set_handler({
					let this = this.clone();
					let mut start = start.clone();
					move |event: PointerEvent| {
						event.prevent_default();
						event.stop_propagation();

						let end = this.capture_position(&event).unwrap();
						let drag = end - start.clone();
						this.set_translation(drag);
						if let Ok(integrate) = this.integrate_on_move.try_borrow() {
							if *integrate {
								start = end;
								this.integrate_transformation();
							}
						}
						this.reposition().unwrap();
					}
				});

				this.move_up_listener.set_handler({
					let this = this.clone();
					move |event: PointerEvent| {
						event.prevent_default();
						event.stop_propagation();

						let mut integrate = false;
						if let Ok(i) = this.integrate_on_move.try_borrow() {
							integrate = *i;
						}
						if !integrate {
							let drag = this.capture_position(&event).unwrap() - start.clone();
							this.set_translation(drag);
							this.integrate_transformation();
						}
						this.reposition();
						this.rerender();
						this.unlock_transformation();
						this.move_move_listener.remove_handler();
						this.move_up_listener.remove_handler();
					}
				});
			}
		});

		this.click_listener.set_handler({
			let this = this.clone();
			move |e| {
				e.stop_propagation();
				e.prevent_default();
				this.on_click.fire(e);
			}
		});
		this.context_listener.set_handler({
			let this = this.clone();
			move |e| {
				e.stop_propagation();
				e.prevent_default();
				this.on_context.fire(e);
			}
		});

		// this.resize_down_listener.set_handler({
		// 	let this = this.clone();
		// 	move |event: PointerEvent| {
		// 		event.prevent_default();
		// 		event.stop_propagation();
		//
		// 		if event.buttons() != 1 { return };
		// 		if !this.lock_transformation() { return };
		// 		event.target().unwrap().dyn_into::<HtmlElement>().unwrap().set_pointer_capture(event.pointer_id());
		// 		let start = this.capture_position(&event).unwrap();
		//
		// 		this.resize_move_listener.set_handler({
		// 			let this = this.clone();
		// 			let start = start.clone();
		// 			move |event: PointerEvent| {
		// 				event.prevent_default();
		// 				event.stop_propagation();
		//
		// 				let drag = this.capture_position(&event).unwrap() - start.clone();
		// 				this.set_scale(drag);
		//
		// 				this.reposition().unwrap();
		// 			}
		// 		});
		//
		// 		this.resize_up_listener.set_handler({
		// 			let this = this.clone();
		// 			move |event: PointerEvent| {
		// 				event.prevent_default();
		// 				event.stop_propagation();
		//
		// 				let drag = this.capture_position(&event).unwrap() - start.clone();
		// 				this.set_scale(drag);
		//
		// 				this.integrate_transformation();
		// 				this.reposition();
		// 				this.rerender();
		// 				this.unlock_transformation();
		// 				this.resize_move_listener.remove_handler();
		// 				this.resize_up_listener.remove_handler();
		// 			}
		// 		});
		// 	}
		// });

		this.reposition().unwrap();
		this.rerender().unwrap();

		this
	}

	pub fn set_integrate_on_move(&self, value: bool) {
		let Ok(locked) = self.transformation_locked.try_borrow() else { return };
		if *locked {
			return;
		}
		let Ok(mut integrate) = self.integrate_on_move.try_borrow_mut() else { return };
		*integrate = value;
	}

	fn selection(&self) -> Result<Option<Bounds>, ()> {
		Ok((self.get_bounds)())
	}

	fn lock_transformation(&self) -> bool {
		let Ok(mut lock) = self.transformation_locked.try_borrow_mut() else { return false };
		if *lock {
			return false;
		};
		*lock = true;
		true
	}
	fn unlock_transformation(&self) {
		if let Ok(mut lock) = self.transformation_locked.try_borrow_mut() {
			*lock = false;
		};
	}

	fn set_translation(&self, total_drag: Vector) -> Result<(), ()> {
		let mut translation = self.translation.try_borrow_mut().or_err(())?;
		*translation = total_drag;
		Ok(())
	}
	fn set_scale(&self, total_drag: Vector) -> Result<(), ()> {
		let bounds = self.selection()?.ok_or(())?;

		let scale_x = Number::max((total_drag.x + bounds.size().x) / bounds.size().x, 0.0);
		let scale_y = Number::max((total_drag.y + bounds.size().y) / bounds.size().y, 0.0);

		let mut scale = self.scale.try_borrow_mut().or_err(())?;
		*scale = Number::max(scale_x, scale_y);
		Ok(())
	}
	fn reset_transformation(&self) -> Result<(), ()> {
		*self.translation.try_borrow_mut().or_err(())? = Vector::zero();
		*self.scale.try_borrow_mut().or_err(())? = 1.0;
		Ok(())
	}
	fn integrate_transformation(&self) -> Result<(), ()> {
		let transformation = {
			let translation = self.translation.try_borrow_mut().or_err(())?;
			let scale = self.scale.try_borrow_mut().or_err(())?;
			let bounds = self.selection()?.ok_or(())?;
			let start = Translation2::new(bounds.start().x, bounds.start().y);
			start
				* convert::<_, Affine2<_>>(Scale2::new(*scale, *scale))
				* start.inverse()
				* Translation2::new(translation.x, translation.y)
		};

		if transformation.try_inverse().is_none() {
			return Err(());
		};

		(self.integrate_transformation)(transformation, self)?;

		self.reset_transformation()
	}

	fn capture_position(&self, event: &PointerEvent) -> Result<Vector, ()> {
		Ok(Vector::new(event.client_x() as f64, event.client_y() as f64))
	}

	/// Adjusts position and size of this component to match display the current selection.
	///
	/// Does not rerender its content (the selection).
	pub fn reposition(&self) -> Result<(), ()> {
		let bounds = self.bounds.try_borrow_mut().or_err(())?;
		let Some(bounds) = bounds.as_ref() else {
			return self.close();
		};

		let translation = self.translation.try_borrow_mut().or_err(())?;
		let scale = self.scale.try_borrow_mut().or_err(())?;

		// calculate the current positioning of canvas
		let start = bounds.start().clone() + translation.clone();
		let size = bounds.size().clone() * *scale;

		// move canvas into position
		self.element.style().set_property("left", format!("{}px", start.x).as_str());
		self.element.style().set_property("top", format!("{}px", start.y).as_str());
		self.element.style().set_property("width", format!("{}px", size.x).as_str());
		self.element.style().set_property("height", format!("{}px", size.y).as_str());

		Ok(())
	}
	/// Rerenders this compontents content (the selection).
	pub fn rerender(&self) -> Result<(), ()> {
		let bounds = self.bounds.try_borrow_mut().or_err(())?;
		let Some(bounds) = bounds.as_ref() else {
			return self.close();
		};

		// adjust canvas to current size of the selection
		self.canvas.set_width(bounds.size().x as u32); // TODO: set resolution according to current zoom level
		self.canvas.set_height(bounds.size().y as u32);

		self.on_render.fire(());

		Ok(())
	}
	pub fn reset(&self) -> Result<(), ()> {
		self.reset_transformation()?;
		*self.bounds.try_borrow_mut().or_err(())? = self.selection()?;
		// TODO: close if no selection
		self.reposition()?;
		Ok(())
	}
	pub fn open(&self) -> Result<(), ()> {
		self.parent.append_child(self.element()).or_err(())?;
		self.reset();
		self.rerender();
		Ok(())
	}
	pub fn close(&self) -> Result<(), ()> {
		self.element().remove();
		self.reset_transformation();
		Ok(())
	}
}

impl ComponentContent for SelectionFrame {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
	fn update(&self) -> Result<()> {
		self.reset().or_err(anyhow!("Failed to reset selection"))
	}
}
