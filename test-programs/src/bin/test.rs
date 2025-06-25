use std::cell::Cell;

wit_bindgen::generate!({
    inline: r"
    package this:wit;
    world w {
        export component-init: func();
    }"
});

const IS_INITIALIZED: Cell<bool> = Cell::new(false);

struct S;
impl Guest for S {
    fn component_init() {
        let before = IS_INITIALIZED.replace(true);
        assert!(!before, "component should only be initialized once");
    }
}

export!(S);

fn main() {
    let initialized = IS_INITIALIZED.get();
    assert!(initialized, "component was not initialized")
}
