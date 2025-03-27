mod app;
pub mod models;
pub mod handlers;
pub mod components;
pub mod dxf;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
