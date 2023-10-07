#![feature(fn_traits)]
#![feature(unboxed_closures)]

use leptos::mount_to_body;

mod components;
mod icons;
mod state;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(components::App)
}
