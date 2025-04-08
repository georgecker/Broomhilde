use app::App;

mod app;
pub mod config;
pub mod organizer;
pub mod ui;

fn main() {
    // CLI
    // -- config for ui
    // -- run for sweap

    let mut terminal = ratatui::init();
    let _ = App::default().run_ui(&mut terminal);
    println!("Hello, world!");
    ratatui::restore();
}
