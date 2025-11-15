pub struct Color {
	value: String,
}

impl Color {
	pub fn value(&self) -> &str {
		self.value.as_str()
	}
}

impl From<&'static str> for Color {
	fn from(value: &'static str) -> Self {
		Self { value: value.to_owned() }
	}
}

impl From<String> for Color {
	fn from(value: String) -> Self {
		Self { value }
	}
}

impl From<Color> for String {
	fn from(value: Color) -> Self {
		value.value
	}
}
