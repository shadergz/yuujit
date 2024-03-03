mod system;
mod memory;

use clap::Parser;

use crate::system::System;

/// Represents the command-line arguments that can be
/// supplied when the compiler is ran
#[derive(Parser)]
pub struct Cli {
    /// Usage --lex / -l: runs the compiler
    /// up to the lexing stage and prints a list of tokens
    #[arg(long, short = 'l')]
    pub lex: bool,

    /// Usage: --parse / -p: runs the compiler
    /// up to the parsing stage and prints the AST
    #[arg(long, short = 'p')]
    pub parse: bool,

    /// The path of the .mocha file to compile
    #[clap(required = true)]
    pub path: String,
}

fn main() {
    let cli = Cli::parse();
    let mut system = System::new(cli.path);
    system.skip_bios();
    system.run_frame();
}
