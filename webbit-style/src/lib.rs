use std::path::{Path, PathBuf};

pub mod builder;
pub use crate::builder::StylesheetBuilder;

#[cfg(test)]
mod tests;

/// Webbit's base stylesheet, without any user additions.
///
/// # Returns
/// The compiled css.
pub fn default_stylesheet() -> Result<String, Box<grass::Error>> {
	StylesheetBuilder::default().build(default_style())
}

impl Default for StylesheetBuilder {
	fn default() -> Self {
		Self::empty().with_src(style_src())
	}
}

// style source file paths:
fn root() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR")).into()
}
fn style_src() -> PathBuf {
	root().join("style")
}
fn default_style() -> PathBuf {
	style_src().join("webbit").join("styles.sass")
}
