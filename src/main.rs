use codex_control_engine::cli;
use colored::*;

#[tokio::main]
async fn main() {
    // Set up graceful interrupt handling
    ctrlc::set_handler(move || {
        println!(
            "\n\n{}",
            "🌙 The ritual is paused. The sacred work awaits your return.".bright_blue()
        );
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Run the CLI
    if let Err(e) = cli::run_cli().await {
        eprintln!("\n{} {}", "❌ Error:".bright_red().bold(), e);
        std::process::exit(1);
    }
}
