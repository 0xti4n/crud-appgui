mod models;
mod application;
mod repository;
mod schema;
mod service;

fn main() {
    let mut gui = application::GUI::new();
    gui.build();
    gui.show();
}
