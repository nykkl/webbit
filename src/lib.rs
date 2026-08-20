use std::{
	cell::{BorrowError, BorrowMutError, RefCell},
	ops::Deref,
};

pub mod common;
pub mod components;
pub mod elements;
pub mod errors;
pub mod events;
pub mod io;

mod component;
pub use component::*;

mod dynamic_component;
pub use dynamic_component::*;

mod component_content;
pub use component_content::*;

mod context;
pub use context::*;

/// A style element holding the css that the calling crate's build script wrote to `OUT_DIR`
/// as [name].
///
/// (Use [on()](crate::elements::on) to mount.)
///
/// You can use `webbit-style` as a build dependency to build a stylesheet using sass and
/// webbit's predefined mixins, components, etc.
/// Have a look at `webbit_style::StylesheetBuilder`.
///
/// # Example
/// ```rust,ignore
/// on(&root, stylesheet!("app.css"));
/// ```
///
/// # Returns
/// A new, unmounted, style element.
#[macro_export]
macro_rules! stylesheet {
	($name:literal) => {
		$crate::elements::style(include_str!(concat!(env!("OUT_DIR"), "/", $name)))
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::errors::{StringErrorHandler, TracksEnvironment};

	#[test]
	fn test() -> Result<(), ()> {
		let (handler, errors) = StringErrorHandler::new("Workspace".to_owned());

		let context = Context::make(7, handler);
		let context = context.clone_for("SelectionFrame");
		let context = context.clone_for("ContextMenu");
		let context = context.clone_for("/click");

		if let Some(mut x) = context.access_mut() {
			if let Some(y) = context.access() {
				*x += *y;
			}
		}

		let errors = errors.get();
		if errors.len() == 1 {
			println!("{}", errors[0]);
			return Ok(());
		}
		Err(())
	}
}
