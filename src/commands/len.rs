use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

/// Get length of given string
#[derive(Debug, Clone, Parser)]
pub struct LenCommand {
	#[arg(long)]
	target: String
}

impl LenCommand {
	pub async fn run(self) -> Result<ExitCode> {
		println!("{}", self.target.len());
		return Ok(ExitCode::SUCCESS)
	}
}
