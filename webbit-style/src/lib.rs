use std::path::{Path, PathBuf};

pub mod builder;
pub use crate::builder::StylesheetBuilder;

#[cfg(test)]
mod tests;

fn root_dir() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR")).into()
}

impl Default for StylesheetBuilder {
	fn default() -> Self {
		Self::empty().with_src(root_dir().join("style"))
	}
}

/// Webbit's base stylesheet, without any user additions.
///
/// # Returns
/// The compiled css.
pub fn default_stylesheet() -> Result<String, Box<grass::Error>> {
	StylesheetBuilder::default().build(root_dir().join("default.sass"))
}
