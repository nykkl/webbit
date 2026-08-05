use web_sys::HtmlDivElement;

use crate::{
	elements::{div, styled},
	events::Group,
	Component, ComponentContent,
};

pub trait ClubMemberFactory<N = ()> {
	type ConstructionArgs;
	type Member: ComponentContent;
	fn make_new(&mut self, group: &mut Group<N>, args: Self::ConstructionArgs) -> Component<Self::Member>;
}

pub struct Club<F: ClubMemberFactory<N>, N = ()> {
	element: HtmlDivElement,
	factory: F,
	members: Vec<Component<F::Member>>,
	group: Group<N>,
}
impl<F: ClubMemberFactory<N>, N> Club<F, N> {
	pub fn new(container_css: &str, factory: F) -> Self {
		let element = styled(div(), container_css);
		Self { element, factory, members: Vec::new(), group: Group::new() }
	}
	pub fn add(&mut self, args: F::ConstructionArgs) -> Result<(), ()> {
		let member = self.factory.make_new(&mut self.group, args);
		member.mount_in(&self.element);
		self.members.push(member);
		Ok(())
	}
}
impl<F: ClubMemberFactory<N>, N> ComponentContent for Club<F, N> {
	fn element(&self) -> &web_sys::Element {
		&self.element
	}
}
