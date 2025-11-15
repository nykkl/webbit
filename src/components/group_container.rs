use web_sys::HtmlDivElement;

use crate::{
	elements::{div, styled},
	events::Group,
	Component, ComponentContent,
};

pub trait GroupContainerElementFactory<N = ()> {
	type ConstructionArgs;
	type Element: ComponentContent;
	fn make_new(&mut self, group: &mut Group<N>, args: Self::ConstructionArgs) -> Component<Self::Element>;
}

pub struct GroupContainer<F: GroupContainerElementFactory<N>, N = ()> {
	element: HtmlDivElement,
	factory: F,
	elements: Vec<Component<F::Element>>,
	group: Group<N>,
}
impl<F: GroupContainerElementFactory<N>, N> GroupContainer<F, N> {
	pub fn new(container_css: &str, factory: F) -> Self {
		let element = styled(div(), container_css);
		Self { element, factory, elements: Vec::new(), group: Group::new() }
	}
	pub fn add(&mut self, args: F::ConstructionArgs) -> Result<(), ()> {
		let element = self.factory.make_new(&mut self.group, args);
		element.mount_in(&self.element);
		self.elements.push(element);
		Ok(())
	}
}
impl<F: GroupContainerElementFactory<N>, N> ComponentContent for GroupContainer<F, N> {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
}
