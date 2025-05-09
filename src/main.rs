mod app;
pub mod models;
pub mod handlers;
pub mod components;
pub mod dxf;
use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

