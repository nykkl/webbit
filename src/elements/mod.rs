use std::{ops::Deref, str::FromStr};

use result_or_err::ResultOrErr;
use wasm_bindgen::JsCast;
use web_sys::{
	window, Element, HtmlCanvasElement, HtmlDivElement, HtmlElement, HtmlHrElement, HtmlInputElement,
	HtmlLabelElement, HtmlTemplateElement, Node,
};

pub fn document() -> web_sys::Document {
	window().unwrap().document().unwrap()
}

/// Mounts the child in the parent element.
///
/// # Example
/// ```rust
/// let parent = div();
/// let child = on(&parent, div()); // create new div in parent div
/// ```
///
/// # Returns
/// The child, now mounted in the parent.
pub fn on<T>(parent: &Element, child: T) -> T
where
	T: AsRef<Node>,
{
	parent.append_child(AsRef::<Node>::as_ref(&child)).unwrap();
	child
}

/// Styles the element with the given css class.
///
/// # Example
/// ```rust
/// let button = styled(div(), "myButtonClass");
/// ```
///
/// # Returns
/// The child, now styled.
pub fn styled<T>(element: T, css: &str) -> T
where
	T: Deref<Target = HtmlElement>,
{
	element.set_class_name(css);
	element
}

/// Creates a new div (that is not mounted anywhere).
///
/// (Use [on()] to mount.)
///
/// # Example
/// ```rust
/// // 1. create new div
/// let my_div = div();
///
/// // 2. create parent div and mount my div in it
/// let parent = div();
/// let my_div = on(&parent, my_div);
///
/// // OR do it in one go:
/// let parent = div();
/// let my_div = on(&parent, div());
/// ```
///
/// # Returns
/// A new, unmounted, div.
pub fn div() -> HtmlDivElement {
	document().create_element("div").unwrap().dyn_into::<HtmlDivElement>().unwrap()
}

pub fn label(text: &str) -> HtmlLabelElement {
	let label = document().create_element("label").unwrap().dyn_into::<HtmlLabelElement>().unwrap();
	label.set_text_content(Some(text));
	label
}

pub fn text(value: &str) -> HtmlInputElement {
	let text = document().create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
	text.set_type("text");
	text.set_value(value);
	text
}

pub fn checkbox(value: bool) -> HtmlInputElement {
	let text = document().create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
	text.set_type("checkbox");
	text.set_checked(value);
	text
}

pub fn color(value: &str) -> HtmlInputElement {
	let color = document().create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
	color.set_type("color");
	color.set_value(value);
	color
}

pub fn slider<T: FromStr + ToString + 'static>(value: &T, min: &T, max: &T, step: &T) -> HtmlInputElement {
	let slider = document().create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
	slider.set_type("range");
	slider.set_min(&min.to_string());
	slider.set_max(&max.to_string());
	slider.set_step(&step.to_string());
	slider.set_value(&value.to_string());
	slider
}

pub fn canvas() -> HtmlCanvasElement {
	document().create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap()
}

pub fn hr() -> HtmlHrElement {
	document().create_element("hr").unwrap().dyn_into::<HtmlHrElement>().unwrap()
}

/// Returns a new node created from the template specified by [id].
///
/// If there is no template with the specified [id] returns `Err(())`.
pub fn from_template(id: &str) -> Result<Node, ()> {
	let template = document().get_element_by_id(id).ok_or(())?.dyn_into::<HtmlTemplateElement>().or_err(())?;
	let new_node = template.content().clone_node_with_deep(true).unwrap();
	Ok(new_node)
}
