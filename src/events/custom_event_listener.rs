use std::{cell::RefCell, mem::replace, ops::DerefMut};

pub struct CustomEventListener<A> {
	// SAFETY: in this struct care is taken to never borrow handler twice
	// the only place where it could be borrowed while outside code is executed is in fire
	// to avoid this, the fire method takes full ownership of the handlers content while it calls it
	// it sets handler_loaned to true and only returns the handler afterwards if it is still true
	// this prevents double borrowing hence we borrow without try_
	// it also allows modifying the handler even during the fire event
	// which may be important for cleanup opertations where remove_handler() should never fail
	handler: RefCell<Option<Box<dyn FnMut(A)>>>,
	// SAFETY: never leaves the struct
	// and in this struct care is taken to never borrow it twice
	// hence we borrow without try_
	handler_loaned: RefCell<bool>,
}

impl<A> CustomEventListener<A> {
	pub fn new() -> Self {
		Self { handler: RefCell::new(None), handler_loaned: RefCell::new(false) }
	}

	pub fn set_handler(
		&self,
		handler: impl FnMut(A) + 'static,
	) -> Result<Option<Box<dyn FnMut(A) + 'static>>, impl FnMut(A) + 'static> {
		let Ok(old) = self.loan_handler() else { return Err(handler) };
		// SAFETY: we just loaned the handler so we know its safe to return it
		self.return_handler(Some(Box::new(handler)));
		Ok(old)
	}

	/// Does the same thing as `set_handler()` just with a different function signature and returns the same handler it is called on.
	///
	/// This is just syntactic sugar, that allows for something like: `DynamicEventListener::new().with_handler().unwrap().forget()`.
	///
	/// Returns self on success and (self, handler) on error.
	pub fn with_handler(
		self,
		handler: impl FnMut(A) + 'static,
	) -> Result<Self, (Self, impl FnMut(A) + 'static)> {
		match self.set_handler(handler) {
			Ok(_) => Ok(self),
			Err(handler) => Err((self, handler)),
		}
	}

	/// Removes the current event handler.
	///
	/// This means events will no longer be handled.
	///
	/// You proably don't need this method. Calling `set_handler` or dropping the listener will remove the current handler automatically:
	/// - So if you want to stop handling events altogether, just drop the listener.
	/// - If you want to change the handler, just call `set_handler` directly.
	pub fn remove_handler(&self) {
		let mut loaned = self.handler_loaned.borrow_mut();
		*loaned = false;
		self.handler.borrow_mut().take();
	}

	pub fn fire(&self, argument: A) -> Result<bool, ()> {
		let mut loan = self.loan_handler()?;

		// SAFETY: taking full ownership of the handler to prevent double borrowing
		let result = match loan.as_mut() {
			Some(mut handler) => {
				handler(argument);
				true
			},
			None => false,
		};

		self.return_handler(loan);
		Ok(result)
	}

	fn loan_handler(&self) -> Result<Option<Box<dyn FnMut(A)>>, ()> {
		let mut loaned = self.handler_loaned.borrow_mut();
		match *loaned {
			true => Err(()),
			false => {
				let mut loan = self.handler.borrow_mut().take();
				*loaned = true;
				Ok(loan)
			},
		}
	}
	fn return_handler(&self, loan: Option<Box<dyn FnMut(A)>>) {
		let mut loaned = self.handler_loaned.borrow_mut();
		if *loaned {
			let mut handler = self.handler.borrow_mut();
			*handler = loan;
		}
		*loaned = false;
	}
}

pub struct CustomRefEventListener<A> {
	handler: RefCell<Option<Box<dyn FnMut(&A)>>>,
}

impl<A> CustomRefEventListener<A> {
	pub fn new() -> Self {
		Self { handler: RefCell::new(None) }
	}

	pub fn set_handler(
		&self,
		handler: impl FnMut(&A) + 'static,
	) -> Result<Option<Box<dyn FnMut(&A) + 'static>>, impl FnMut(&A) + 'static> {
		let Ok(mut original) = self.handler.try_borrow_mut() else { return Err(handler) };

		let original = original.deref_mut();
		let new = Box::new(handler);

		Ok(replace(original, Some(new)))
	}

	/// Does the same thing as `set_handler()` just with a different function signature and returns the same handler it is called on.
	///
	/// This is just syntactic sugar, that allows for something like: `DynamicEventListener::new().with_handler().unwrap().forget()`.
	///
	/// Returns self on success and (self, handler) on error.
	pub fn with_handler(
		self,
		handler: impl FnMut(&A) + 'static,
	) -> Result<Self, (Self, impl FnMut(&A) + 'static)> {
		match self.set_handler(handler) {
			Ok(_) => Ok(self),
			Err(handler) => Err((self, handler)),
		}
	}

	/// Removes the current event handler.
	///
	/// This means events will no longer be handled.
	///
	/// You proably don't need this method. Calling `set_handler` or dropping the listener will remove the current handler automatically:
	/// - So if you want to stop handling events altogether, just drop the listener.
	/// - If you want to change the handler, just call `set_handler` directly.
	pub fn remove_handler(&self) -> Result<Option<Box<dyn FnMut(&A) + 'static>>, ()> {
		let Ok(mut handler) = self.handler.try_borrow_mut() else { return Err(()) };

		Ok(handler.take())
	}

	pub fn fire(&self, argument: &A) -> Result<bool, ()> {
		let Ok(mut handler) = self.handler.try_borrow_mut() else { return Err(()) };
		let Some(handler) = handler.deref_mut() else { return Ok(false) };

		handler(argument);
		Ok(true)
	}
}
