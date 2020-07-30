#[macro_use]
extern crate log;

use hazel_rs::Application;

struct Sandbox();

impl Application for Sandbox {
    fn new() -> Self {
        Sandbox {}
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "sandbox=debug,hazel_rs=debug");
    env_logger::init();
    warn!("This is an example message.");

    let sandbox = Sandbox::new();
    hazel_rs::start_app(sandbox);
}