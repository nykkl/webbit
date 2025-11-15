use std::error::Error;

use wasm_bindgen::JsValue;
use web_sys::console;

use crate::errors::{Environment, ErrorHandler, TracksEnvironment};

#[derive(Default)]
pub struct ConsoleErrorHandler {
	environment: String,
}
impl ConsoleErrorHandler {
	pub fn new(environment: String) -> Self {
		Self { environment }
	}
}
impl<E: Error> ErrorHandler<E> for ConsoleErrorHandler {
	fn handle(&self, err: E) {
		console::log_1(&JsValue::from(format!("{}\nin {}", err, self.environment)));
	}
}
impl TracksEnvironment for ConsoleErrorHandler {
	type EnvironmentType = String;
	fn environment(&self) -> &Self::EnvironmentType {
		&self.environment
	}
	fn environment_mut(&mut self) -> &mut Self::EnvironmentType {
		&mut self.environment
	}
	fn clone_for<'e>(&self, sub_environment: <Self::EnvironmentType as Environment>::ExtensionType<'e>) -> Self {
		let mut environment = self.environment.clone();
		environment.extend_path(sub_environment);
		Self { environment }
	}
}
