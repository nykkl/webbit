use super::*;

fn compile_str(sass: &str) -> String {
	grass::from_string(sass, &grass::Options::default().load_path(style_src())).unwrap()
}

#[test]
fn default_stylesheet_compiles() {
	let css = default_stylesheet().unwrap();
	assert!(css.contains(".bar-button"));
	assert!(css.contains(".selection-frame"));
	assert!(css.contains(".context-menu"));
}

#[test]
fn variables_are_configurable() {
	let css = compile_str("@use 'webbit/components' with ($selection-color: red);");
	assert!(css.contains("dashed red"));
}

#[test]
fn mixins_are_usable_downstream() {
	let css = compile_str("@use 'webbit/base'; .my-panel { @include base.flexFill; }");
	assert!(css.contains(".my-panel"));
	assert!(css.contains("flex: 1 1 100%"));
}

/// A library layering on webbit adds its own sources, and both are available to `@use`.
#[test]
fn added_sources_are_usable() {
	let dir = std::env::temp_dir().join("webbit-style-layer-test");
	std::fs::create_dir_all(dir.join("layer")).unwrap();
	std::fs::write(dir.join("layer").join("extra.sass"), "@mixin highlight\n\tcolor: red\n").unwrap();
	let entry = dir.join("app.sass");
	let sass =
		"@use 'webbit/base'\n@use 'layer/extra'\n\n.thing\n\t@include base.fill\n\t@include extra.highlight\n";
	std::fs::write(&entry, sass).unwrap();

	let css = StylesheetBuilder::default().with_src(&dir).build(&entry).unwrap();
	assert!(css.contains("color: red"));
	assert!(css.contains("width: 100%"));

	std::fs::remove_dir_all(&dir).unwrap();
}
