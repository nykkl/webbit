use std::{
	cell::{Ref, RefCell, RefMut},
	rc::Rc,
};

use crate::errors::{ContextInUseError, Environment, ErrorHandler, TracksEnvironment};

/// To communicate with stuff that our component is not responsible for we probably want to use an Rc<RefCell<>>.
/// Calling it `Reference` looks nicer.
pub type Reference<T> = Rc<RefCell<T>>;

pub struct Context<T, E: ErrorHandler<ContextInUseError>> {
	data: Reference<T>,
	pub error_handler: E,
}
impl<T, H: ErrorHandler<ContextInUseError>> Context<T, H> {
	pub fn new(data: Reference<T>, error_handler: H) -> Self {
		Self { data, error_handler }
	}
	pub fn make(data: T, error_handler: H) -> Self {
		Self::new(Rc::new(RefCell::new(data)), error_handler)
	}

	pub fn access<'b>(&'b self) -> Option<Ref<'b, T>> {
		let result = self.data.try_borrow();
		match result {
			Ok(v) => Some(v),
			Err(e) => {
				self.error_handler.handle(ContextInUseError::Borrowed(e));
				None
			},
		}
	}
	pub fn access_mut<'b>(&'b self) -> Option<RefMut<'b, T>> {
		let result = self.data.try_borrow_mut();
		match result {
			Ok(v) => Some(v),
			Err(e) => {
				self.error_handler.handle(ContextInUseError::BorrowedMut(e));
				None
			},
		}
	}
	pub fn access_or<'b, Err>(&'b self, err: Err) -> Result<Ref<'b, T>, Err> {
		self.access().ok_or(err)
	}
	pub fn access_mut_or<'b, Err>(&'b self, err: Err) -> Result<RefMut<'b, T>, Err> {
		self.access_mut().ok_or(err)
	}
}
impl<T, H, E> ErrorHandler<E> for Context<T, H>
where
	H: ErrorHandler<E> + ErrorHandler<ContextInUseError>,
{
	fn handle(&self, err: E) {
		self.error_handler.handle(err);
	}
}
impl<T, H: ErrorHandler<ContextInUseError>> TracksEnvironment for Context<T, H> {
	type EnvironmentType = <H as TracksEnvironment>::EnvironmentType;
	fn environment(&self) -> &Self::EnvironmentType {
		self.error_handler.environment()
	}
	fn environment_mut(&mut self) -> &mut Self::EnvironmentType {
		self.error_handler.environment_mut()
	}
	fn clone_for<'e>(
		&self,
		environment: <<H as TracksEnvironment>::EnvironmentType as Environment>::ExtensionType<'e>,
	) -> Self {
		Self { data: self.data.clone(), error_handler: self.error_handler.clone_for(environment) }
	}
}
