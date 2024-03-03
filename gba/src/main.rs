mod system;
mod memory;

use clap::Parser;

use crate::system::System;

#[derive(Parser)]
pub struct Cli {
    #[clap(required = true)]
    pub path: String,
}

fn main() {
    let cli = Cli::parse();
    let mut system = System::new(cli.path);
    system.skip_bios();
    system.run_frame();
}
