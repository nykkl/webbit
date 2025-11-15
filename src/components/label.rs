use web_sys::HtmlLabelElement;

use crate::{
	elements::{label, styled},
	ComponentContent,
};

pub struct Label {
	root: HtmlLabelElement,
}
impl Label {
	pub fn new(text: &str, css: &str) -> Self {
		let root = styled(label(text), css);
		Self { root }
	}
	pub fn set_text(&self, text: Option<&str>) {
		self.root.set_text_content(text);
	}
}
impl ComponentContent for Label {
	fn element(&self) -> &web_sys::Element {
		&self.root
	}
}
