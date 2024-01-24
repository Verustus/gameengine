#[macro_use]
extern crate glium;

macro_rules! inner_path {
    ($path:expr) => {
        { std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($path) }
    };
}

pub mod graphics;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    graphics::opengl::window::start();
}