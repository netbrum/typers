use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
pub struct Args {
    #[arg(short, long, default_value_t = 24)]
    pub words: usize,
}
