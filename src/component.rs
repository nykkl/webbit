use std::{ops::Deref, rc::Rc};

use anyhow::Result;
use web_sys::Element;

use super::ComponentContent;

/// A ui component.
/// Meaning a struct that represents a certain piece of ui (and removes that when it is dropped).
///
/// # Motivation
/// Meant to make ui easier and more flexible by keeping the ui content behind an Rc and mostly using immutable methods for the component (that only require &self).
/// The reasoning behind this is that the actual ui elements don't require a mutable reference to change them anyway.
///
/// # Idea
/// To that end a compponent is meant to handle mainly 4 types of content:
/// - ui `Element`s: only require readonly reference (&self) to manipulate; can just be cloned
/// - `EventListener`s: to handle events on this components elements (deregister when dropped with the component)
/// - owned data (`RefCell` if mutable): the state of this component; meant to be accessed by this component (can be accessed by child components via Rc<this component>)
/// - `Reference`s: to access stuff outside of this component (parts of the model not managed by this component) (not high performance; use sparsely)
///
/// So all of that should be mostly accomplishable with immutable methods (&self).
/// This isn't efficient (reference-counting and runtime borrow-checking) but easier.
/// And in the applications this is meant for (menues, ...) that performance shouldn`t matter.
///
/// # Usage
/// - make Component<YourContent> and store as long as you need the component
/// - store EventListeners in YourContent (they are automatically deregistered when the component is dropped)
/// - share YourContent with other code parts (it's an Rc); your component will still unmount when Component is dropped
/// - store child Components in YourContent
///		- they are dropped when YourContent is dropped
///		- YourContent is dropped when the Component is dropped AND all other Rcs to YourContent are dropped
/// - use your component from:
///		- where your Component is stored
///		- whereever you have an Rc to YourContent
///
/// # Implementing YourContent
/// - store the ui elements you use so you can unmount them
/// - register events using `EventListener`s and store them to keep the events registered
/// - implement unmount() such that it:
///		- unmounts your ui elements
///		- deregisters your `EventListener`s
pub struct Component<C: ComponentContent> {
	content: Rc<C>,
}
impl<C: ComponentContent> Component<C> {
	/// If you can (if you don't need to reference the content from the outside) use `Component::make` instead.
	/// That makes sure no one else has a reference to the content, so there won't be any interferences when the content is cleaned up on `Component::drop()`.
	fn new(content: C) -> Self {
		Self::make(content)
	}
	pub fn make(content: C) -> Self {
		Self::make_sharable(Rc::new(content))
	}
	pub fn make_sharable(content: Rc<C>) -> Self {
		Self { content }
	}

	pub fn mount_in(&self, element: &Element) -> Result<()> {
		element.append_child(self.content.element()).or(Err(anyhow::anyhow!("Failed to mount component")))?;

		Ok(())
	}

	pub fn update(&self) -> Result<()> {
		self.content.update()?;

		Ok(())
	}

	pub fn unmount(&self) -> Result<()> {
		self.content.element().remove();

		Ok(())
	}
}

impl<C: ComponentContent> Drop for Component<C> {
	fn drop(&mut self) {
		self.content.destroy().unwrap();
		self.content.element().remove();
	}
}

impl<T: ComponentContent> Deref for Component<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.content
	}
}
