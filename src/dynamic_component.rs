use anyhow::Result;
use web_sys::Element;

use super::{Component, ComponentContent};

/// Trait for using a Component as a trait object.
pub trait DynamicComponent {
	fn mount_in(&self, element: &Element) -> Result<()>;
	fn update(&self) -> Result<()>;
	fn unmount(&self) -> Result<()>;
}
impl<C: ComponentContent> DynamicComponent for Component<C> {
	fn mount_in(&self, element: &Element) -> Result<()> {
		Component::mount_in(self, element)
	}
	fn update(&self) -> Result<()> {
		Component::update(self)
	}
	fn unmount(&self) -> Result<()> {
		Component::unmount(self)
	}
}
