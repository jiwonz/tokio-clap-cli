use std::process::ExitCode;

use tokio_clap_cli::Cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> ExitCode {
    match Cli::new().run().await {
		Ok(code) => code,
		Err(err) => {
			eprintln!("{:?}", err);
			ExitCode::FAILURE
		}
	}
}
