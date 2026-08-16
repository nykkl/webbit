use std::path::{Path, PathBuf};

pub mod builder;
pub use crate::builder::StylesheetBuilder;

#[cfg(test)]
mod tests;

fn style_src() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR")).join("style")
}

impl Default for StylesheetBuilder {
	fn default() -> Self {
		Self::empty().with_src(style_src())
	}
}

/// Webbit's base stylesheet, without any user additions.
///
/// # Returns
/// The compiled css.
pub fn default_stylesheet() -> Result<String, Box<grass::Error>> {
	StylesheetBuilder::default().build(style_src().join("webbit").join("styles.sass"))
}
