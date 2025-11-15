use std::fmt::Display;

pub trait ErrorHandler<E>: TracksEnvironment {
	fn handle(&self, err: E);
}
pub trait TracksEnvironment {
	// TracksEnvironment needs to be a separate trait
	// so that every type that implements ErrorHandler<E> (possibly multiple times) can implement TracksEnvironment **ONLY ONCE**
	// otherwise types that implement ErrorHandler<E> multiple times (to handle different types of errors) could have multiple Environments
	type EnvironmentType: Environment;
	fn environment(&self) -> &Self::EnvironmentType;
	fn environment_mut(&mut self) -> &mut Self::EnvironmentType;
	fn clone_for<'e>(&self, sub_environment: <Self::EnvironmentType as Environment>::ExtensionType<'e>) -> Self;
}
pub trait Environment: Clone + Display {
	type ExtensionType<'e>;
	fn extend_path<'e>(&mut self, sub_environment: Self::ExtensionType<'e>);
}

pub trait ErrorHandling<T, E> {
	fn handle(self, e: &impl ErrorHandler<E>) -> Option<T>;
}
impl<T, E> ErrorHandling<T, E> for Result<T, E> {
	fn handle(self, handler: &impl ErrorHandler<E>) -> Option<T> {
		match self {
			Ok(value) => Some(value),
			Err(error) => {
				handler.handle(error);
				None
			},
		}
	}
}
