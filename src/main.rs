mod app;
pub mod models;
pub mod handlers;
pub mod dxf;





use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
