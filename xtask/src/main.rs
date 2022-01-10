use clap::Parser;
use std::path::PathBuf;

/// The command line interface for building korx
#[derive(Debug, Parser)]
#[clap(about, version)]
pub struct Options {
    /// The path to the kernel binary.
    #[clap(parse(from_os_str))]
    pub kernel_bin: PathBuf,

    /// The command to execute.
    #[clap(subcommand)]
    pub cmd: Option<Subcommand>,
}

impl Options {
    pub fn build_kernel(&self) {
        println!("building kernel from binary {}", self.kernel_bin.display());
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Run the kernel inside QEMU
    Run,
}

impl Subcommand {
    pub fn run(self) {
        match self {
            Subcommand::Run => {
                println!("running kernel");
            }
        }
    }
}

fn main() {
    let opts = Options::parse();

    opts.build_kernel();

    if let Some(cmd) = opts.cmd {
        cmd.run();
    }
}
