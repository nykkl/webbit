use std::{cell::RefCell, mem::take};

use web_sys::HtmlDivElement;

use crate::{
	elements::{div, styled},
	events::Group,
	Component, ComponentContent,
};

/// Builds the elements of a [`MutableGroupContainer`].
///
/// The group is taken as `&Group<N>`: all of [`Group`]'s methods take `&self`.
pub trait MutableGroupContainerElementFactory<N = ()> {
	type ConstructionArgs;
	type Element: ComponentContent;
	fn make_new(&mut self, group: &Group<N>, args: Self::ConstructionArgs) -> Component<Self::Element>;
}

/// The mutable sibling of `webbit::components::GroupContainer`: an ordered list of factory-built
/// components sharing a [`Group`], whose contents can still be changed after the container is
/// wrapped in a [`Component`]. A `Component` only hands out shared references, so all methods
/// take `&self`.
///
/// Dropping elements runs their destructors, which can run caller code -- including an element's
/// own event handler ending up dropping that very element (a click that rebuilds the container,
/// say). Such a handler must make mutating the container the last thing it does.
pub struct MutableGroupContainer<F: MutableGroupContainerElementFactory<N>, N = ()> {
	element: HtmlDivElement,
	factory: RefCell<F>,
	elements: RefCell<Vec<Component<F::Element>>>,
	group: Group<N>,
}
impl<F: MutableGroupContainerElementFactory<N>, N> MutableGroupContainer<F, N> {
	pub fn new(container_css: &str, factory: F) -> Self {
		Self {
			element: styled(div(), container_css),
			factory: RefCell::new(factory),
			elements: RefCell::new(Vec::new()),
			group: Group::new(),
		}
	}

	/// Builds one element and appends it.
	pub fn add(&self, args: F::ConstructionArgs) -> Result<(), ()> {
		// `elements` is not borrowed across `make_new`: the factory runs caller code that may
		// touch this container. `factory` is borrowed, so re-entering `add` fails instead of
		// aliasing.
		let element = self.factory.try_borrow_mut().or(Err(()))?.make_new(&self.group, args);

		element.mount_in(&self.element).or(Err(()))?;
		self.elements.try_borrow_mut().or(Err(()))?.push(element);

		Ok(())
	}

	pub fn clear(&self) -> Result<(), ()> {
		// Take the elements out before dropping them: their destructors can run caller code,
		// which must not find `elements` still borrowed.
		let elements = take(&mut *self.elements.try_borrow_mut().or(Err(()))?);
		drop(elements);

		Ok(())
	}
}
impl<F: MutableGroupContainerElementFactory<N>, N> ComponentContent for MutableGroupContainer<F, N> {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
}
