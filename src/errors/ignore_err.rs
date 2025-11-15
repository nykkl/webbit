use crate::errors::{Environment, ErrorHandler, TracksEnvironment};

#[derive(Default)]
pub struct IgnoreErr {
	environment: String,
}
impl IgnoreErr {
	pub fn new(environment: String) -> Self {
		Self { environment }
	}
}
impl<E> ErrorHandler<E> for IgnoreErr {
	fn handle(&self, err: E) {}
}
impl TracksEnvironment for IgnoreErr {
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
