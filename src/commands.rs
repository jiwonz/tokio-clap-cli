use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod len;

pub use len::LenCommand;

#[derive(Debug, Clone, Subcommand)]
pub enum CliSubcommand {
	Len(LenCommand)
}

/// Example CLI using tokio and clap
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
	#[clap(subcommand)]
	subcommand: CliSubcommand,
}

impl Cli {
	pub fn new() -> Self {
		Self::parse()
	}

	pub async fn run(self) -> Result<ExitCode> {
		match self.subcommand {
			CliSubcommand::Len(cmd) => cmd.run().await
		}
	}
}
