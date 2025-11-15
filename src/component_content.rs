use std::rc::Rc;

use anyhow::{Ok, Result};
use web_sys::Element;

/// Implement this to be able to make ui components from your type.
///
/// Like so:
/// ```rust
/// pub struct Toolbar { ... }
/// impl Toolbar {
///		pub fn new() -> Rc<Self> {
///			let this = Rc::new(Self {
///				...
///			});
///
///			...
///
///			this
///		}
///		...
/// }
///
/// // implementation
/// impl ComponentContent for Toolbar { ... }
///
/// // usage
/// pub struct App {
///		toolbar: Component<Toolbar>,
///		...
/// }
/// impl App {
///		pub fn new() -> Self {
///			Self {
///				toolbar: Component::new(Toolbar::new()),
///				...
///			}
///		}
///		...
/// }
/// ```
pub trait ComponentContent {
	// fn new(params: I) -> (Self, Node);

	// /// Mounts the ui of this component in the given element.
	// fn mount_elements_in(&mut self, element: &Element);
	// /// Unmounts the ui of this component.
	// fn unmount_elements(&mut self);
	// /// Registers the events for this component.
	// fn register_events(&mut self, this: Rc<RefCell<Self>>);
	// /// Deregisters the events for this component.
	// fn deregister_events(&mut self, this: Rc<RefCell<Self>>);

	// fn create(&mut self, this: Rc<RefCell<Self>>);

	/// The element that represents this Component in the DOM.
	/// This should always return the same element.
	fn element(&self) -> &Element;
	fn mount_in(&self, element: &Element) -> Result<()> {
		element.append_child(self.element()).or(Err(anyhow::anyhow!("Failed to mount component")))?;

		Ok(())
	}
	fn update(&self) -> Result<()> {
		Ok(())
	}
	/// Clean up before the component is dropped.
	/// This is to e.g. unregister events that might conatain a circular reference to this component.
	fn destroy(&self) -> Result<()> {
		Ok(())
	}
}
