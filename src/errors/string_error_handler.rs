use std::{cell::RefCell, error::Error, rc::Rc};

use crate::errors::{Environment, ErrorHandler, TracksEnvironment};

pub struct ErrorAccessToken {
	errors: Rc<RefCell<Vec<String>>>,
}
impl ErrorAccessToken {
	pub fn get(&self) -> Vec<String> {
		self.errors.borrow().clone()
	}
}

#[derive(Default)]
pub struct StringErrorHandler {
	environment: String,
	errors: Rc<RefCell<Vec<String>>>,
}
impl StringErrorHandler {
	pub fn new(environment: String) -> (Self, ErrorAccessToken) {
		let errors = Rc::new(RefCell::new(Vec::new()));
		(Self { environment, errors: errors.clone() }, ErrorAccessToken { errors })
	}
}
impl<E: Error> ErrorHandler<E> for StringErrorHandler {
	fn handle(&self, err: E) {
		let mut errors = self.errors.borrow_mut();
		errors.push(format!("ERROR: \"{}\"\n\tin {}", err, self.environment));
	}
}
impl TracksEnvironment for StringErrorHandler {
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
		Self { environment, errors: self.errors.clone() }
	}
}
