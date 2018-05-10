#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

//TODO: replace with autogen headers from https://github.com/rustwasm/wasm-bindgen/issues/42
#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    type HTMLDocument;
    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    type Element;

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log("Look ma, I'm logging!");

    let val = document.createElement("p");
    val.set_inner_html("Hello from Rust!");
    document.body().append_child(val);

    alert(&format!("Hello, {}!", name));
}