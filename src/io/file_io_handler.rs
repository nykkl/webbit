use std::rc::Rc;

use js_sys::{Boolean, Function, Uint8Array};
use result_or_err::ResultOrErr;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::console;

use crate::events::CustomEventListener;

pub struct FileIOHandler {
	load: Function,
	save: Function,
	load_listener: Closure<dyn FnMut(JsValue, Uint8Array)>,
	save_listener: Closure<dyn FnMut(Boolean)>,
	on_load: Rc<CustomEventListener<Result<Option<Vec<u8>>, ()>>>,
	on_save: Rc<CustomEventListener<bool>>,
}
impl FileIOHandler {
	pub fn new(load: Function, save: Function) -> Self {
		let on_load = Rc::new(CustomEventListener::new());
		let load_listener = Closure::<dyn FnMut(JsValue, Uint8Array)>::new({
			let on_load = on_load.clone();
			move |err: JsValue, data: Uint8Array| {
				let result;
				if !err.is_falsy() {
					result = Err(());
				} else if data.is_falsy() {
					result = Ok(None);
				} else {
					result = Ok(Some(data.to_vec()));
				}
				if on_load.fire(result).is_err() {
					console::log_1(&JsValue::from("FileIOHandler: firing on_load failed."));
				}
			}
		});
		let on_save = Rc::new(CustomEventListener::new());
		let save_listener = Closure::<dyn FnMut(Boolean)>::new({
			let on_save = on_save.clone();
			move |success: Boolean| {
				if on_save.fire(success.is_truthy()).is_err() {
					console::log_1(&JsValue::from("FileIOHandler: firing on_save failed."));
				}
			}
		});
		Self { load, save, load_listener, save_listener, on_load, on_save }
	}
	pub fn load(&self) -> Result<(), ()> {
		self.load.call1(&JsValue::null(), self.load_listener.as_ref().unchecked_ref()).or_err(())?;
		Ok(())
	}
	pub fn save(&self, data: &[u8]) -> Result<(), ()> {
		self
			.save
			.call2(&JsValue::null(), &Uint8Array::from(data), self.save_listener.as_ref().unchecked_ref())
			.or_err(())?;
		Ok(())
	}
	pub fn on_load(&self) -> &CustomEventListener<Result<Option<Vec<u8>>, ()>> {
		&self.on_load
	}
	pub fn on_save(&self) -> &CustomEventListener<bool> {
		&self.on_save
	}
}
