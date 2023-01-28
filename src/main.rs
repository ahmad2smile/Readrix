mod app;
mod posts;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
