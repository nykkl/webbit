use crate::errors::Environment;

impl Environment for String {
	type ExtensionType<'e> = &'e str;
	fn extend_path<'e>(&mut self, sub_environment: Self::ExtensionType<'e>) {
		self.push_str("\n\t-> ");
		self.push_str(sub_environment);
	}
}
