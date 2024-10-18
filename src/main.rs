mod app;
mod args;

use app::App;
pub use args::Args;
use clap::Parser;
use std::io;

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    let app_result = App::new(args).run(&mut terminal);
    ratatui::restore();
    app_result
}
