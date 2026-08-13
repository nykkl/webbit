use std::path::{Path, PathBuf};

fn style_src() -> PathBuf {
	Path::new(env!("CARGO_MANIFEST_DIR")).join("style")
}

/// Builds a stylesheet from sass, with webbit's sass available to `@use` as `webbit/<file>`.
///
/// Use this in the `build.rs` of the crate that needs a stylesheet, with this crate as a
/// build-dependency, and write the css to `OUT_DIR` to then include it using `webbit::stylesheet!()`.
///
/// # Usage
/// ## app/build.rs
/// ```rust,no_run
/// let css = webbit_style::StylesheetBuilder::default().build("style/app.sass").unwrap();
/// std::fs::write(format!("{}/app.css", std::env::var("OUT_DIR").unwrap()), css).unwrap();
/// println!("cargo:rerun-if-changed=style");
/// ```
/// ## app/src/lib.rs
/// ```rust,ignore
/// on(&root, webbit::stylesheet!("app.css"));
/// ```
/// ## app/style/app.sass
/// ```sass
/// @use 'webbit/base'
/// @use 'webbit/styles'
///
/// .app
/// 	@include base.flexContainer
/// 	@include base.fill
/// ```
pub struct StylesheetBuilder {
	options: grass::Options<'static>,
}
impl Default for StylesheetBuilder {
	fn default() -> Self {
		Self { options: grass::Options::default().load_path(style_src()) }
	}
}
impl StylesheetBuilder {
	/// Makes the sass in [dir] available to `@use` as well.
	///
	/// # Example
	/// ```rust,ignore
	/// pub fn my_stylesheet_builder() -> StylesheetBuilder {
	/// 	let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("style");
	/// 	webbit_style::StylesheetBuilder::default().with_src(src)
	/// }
	/// ```
	pub fn with_src(mut self, dir: impl AsRef<Path>) -> Self {
		self.options = self.options.load_path(dir);
		self
	}

	/// Compiles the sass file at [entry_point].
	///
	/// # Returns
	/// The compiled css.
	pub fn build(&self, entry_point: impl AsRef<Path>) -> Result<String, Box<grass::Error>> {
		grass::from_path(entry_point, &self.options)
	}
}

/// Webbit's base stylesheet, without any user additions.
///
/// # Returns
/// The compiled css.
pub fn default_stylesheet() -> Result<String, Box<grass::Error>> {
	StylesheetBuilder::default().build(style_src().join("webbit").join("styles.sass"))
}

#[cfg(test)]
mod tests;
