use frontend::ClientApp;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<ClientApp>::new().hydrate();
}