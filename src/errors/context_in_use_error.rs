use std::{
	cell::{BorrowError, BorrowMutError},
	error::Error,
	fmt::Display,
};

#[derive(Debug)]
pub enum ContextInUseError {
	Borrowed(BorrowError),
	BorrowedMut(BorrowMutError),
}
impl Error for ContextInUseError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			ContextInUseError::Borrowed(borrow_error) => Some(borrow_error),
			ContextInUseError::BorrowedMut(borrow_mut_error) => Some(borrow_mut_error),
		}
	}
}
impl Display for ContextInUseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "context is already in use")
	}
}
