use std::cell::Cell;

wit_bindgen::generate!({
    inline: r"
    package this:wit;
    world w {
        export component-init: func();
    }"
});

// rustc won't allow using a a Cell in a static, but since this is a component (which is always
// single-threaded) we can ignore this and mark it send and sync.
struct MakeSendSync<T>(T);
unsafe impl<T> Send for MakeSendSync<T> {}
unsafe impl<T> Sync for MakeSendSync<T> {}

static IS_INITIALIZED: MakeSendSync<Cell<bool>> = MakeSendSync(Cell::new(false));

struct S;
impl Guest for S {
    fn component_init() {
        let before = IS_INITIALIZED.0.replace(true);
        assert!(!before, "component should only be initialized once");
    }
}

export!(S);

fn main() {
    let initialized = IS_INITIALIZED.0.get();
    assert!(initialized, "component was not initialized")
}
