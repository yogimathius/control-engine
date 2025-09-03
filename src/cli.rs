use crate::{CodexEngine, CodexError};
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(
    name = "codex",
    about = "🔮 Codex Control Engine - A symbolic runtime for inner transformation",
    long_about = "The Codex Control Engine is a sacred interface for executing rituals, \
                 evolving archetypal states, and reflecting on the mysteries of transformation."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute a symbolic ritual
    #[command(name = "ritual")]
    Ritual {
        #[command(subcommand)]
        action: RitualCommands,
    },
    /// View current symbolic state
    #[command(name = "state")]
    State {
        #[command(subcommand)]
        action: StateCommands,
    },
    /// Seek AI reflection on the last ritual
    #[command(name = "reflect")]
    Reflect,
    /// List available rituals
    #[command(name = "list")]
    List,
    /// Initialize or reset the symbolic state
    #[command(name = "init")]
    Init {
        /// Force reinitialization even if state exists
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum RitualCommands {
    /// Run a named ritual
    #[command(name = "run")]
    Run {
        /// Name of the ritual to execute
        name: String,
    },
}

#[derive(Subcommand)]
pub enum StateCommands {
    /// View the current symbolic state
    #[command(name = "view")]
    View,
    /// Show a summary of the current state
    #[command(name = "summary")]
    Summary,
}

pub async fn run_cli() -> Result<(), CodexError> {
    let cli = Cli::parse();

    // Print the sacred banner
    print_banner();

    let mut engine = CodexEngine::new()?;

    match cli.command {
        Commands::Ritual { action } => match action {
            RitualCommands::Run { name } => {
                execute_ritual(&mut engine, &name).await?;
            }
        },
        Commands::State { action } => match action {
            StateCommands::View => {
                engine.view_state();
            }
            StateCommands::Summary => {
                show_state_summary(&engine);
            }
        },
        Commands::Reflect => {
            engine.reflect().await?;
        }
        Commands::List => {
            engine.list_available_rituals();
        }
        Commands::Init { force } => {
            initialize_system(&mut engine, force)?;
        }
    }

    Ok(())
}

fn print_banner() {
    let banner = r#"
    ╔══════════════════════════════════════════════════════════╗
    ║                                                          ║
    ║               🔮 CODEX CONTROL ENGINE 🔮                ║
    ║                                                          ║
    ║           A Sacred Runtime for Transformation            ║
    ║                                                          ║
    ║      "Every ritual is a doorway. Every command a chant." ║
    ║                                                          ║
    ╚══════════════════════════════════════════════════════════╝
    "#;

    println!("{}", banner.bright_purple());
}

async fn execute_ritual(engine: &mut CodexEngine, ritual_name: &str) -> Result<(), CodexError> {
    println!(
        "\n{}",
        format!("🌟 Preparing to invoke ritual: {}", ritual_name)
            .bright_cyan()
            .bold()
    );

    match engine.execute_ritual(ritual_name).await {
        Ok(_result) => {
            println!(
                "\n{}",
                "🎭 Ritual execution complete. Use 'codex reflect' to gain deeper insights."
                    .bright_green()
            );
            Ok(())
        }
        Err(CodexError::RitualNotFound { name }) => {
            println!(
                "\n{}",
                format!("❌ Ritual '{}' not found.", name).bright_red()
            );
            println!("{}", "Available rituals:".bright_yellow());
            engine.list_available_rituals();
            Err(CodexError::RitualNotFound { name })
        }
        Err(e) => {
            println!(
                "\n{}",
                format!("❌ Ritual execution failed: {}", e).bright_red()
            );
            Err(e)
        }
    }
}

fn show_state_summary(engine: &CodexEngine) {
    let state = engine.get_state();

    println!("\n{}", "📊 SYMBOLIC STATE SUMMARY".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_purple());

    println!("{}", state.get_activation_summary().white());

    if !state.unresolved_symbols.is_empty() {
        println!(
            "\n{} {}",
            "🔍 Unresolved Symbols:".bright_yellow(),
            state.unresolved_symbols.len().to_string().bright_red()
        );
    }

    if !state.active_transformations.is_empty() {
        println!(
            "{} {}",
            "🔄 Active Transformations:".bright_blue(),
            state
                .active_transformations
                .len()
                .to_string()
                .bright_green()
        );
    }

    println!(
        "\n{} {}",
        "🎯 Evolution Cycle:".bright_magenta(),
        state.evolution_cycle.to_string().bright_cyan()
    );

    println!("{}", "═".repeat(50).bright_purple());
}

fn initialize_system(engine: &mut CodexEngine, force: bool) -> Result<(), CodexError> {
    if !force {
        let state = engine.get_state();
        if !state.archetypes.is_empty() {
            println!(
                "{}",
                "🔮 Symbolic state already exists. Use --force to reinitialize.".bright_yellow()
            );
            return Ok(());
        }
    }

    println!(
        "{}",
        "🌟 Initializing the Codex Control Engine..."
            .bright_cyan()
            .bold()
    );

    engine.save_state()?;

    println!(
        "{}",
        "✨ Primordial archetypes and energies have been established.".bright_green()
    );
    println!(
        "{}",
        "🎭 The system is ready for ritual work.".bright_magenta()
    );

    Ok(())
}

pub fn print_usage_examples() {
    let examples = r#"
🔮 CODEX USAGE EXAMPLES:

Basic Commands:
  codex init                           # Initialize the system
  codex list                          # Show available rituals
  codex state view                    # View detailed symbolic state
  codex state summary                 # Quick state overview

Ritual Execution:
  codex ritual run shadow_integration    # Integrate shadow aspects
  codex ritual run energy_attunement     # Harmonize energies
  codex ritual run archetype_invocation  # Activate archetypes
  codex ritual run void_contemplation    # Enter emptiness

Reflection:
  codex reflect                       # AI reflection on last ritual

Workflow Example:
  codex init                          # 1. Initialize system
  codex state view                    # 2. Examine starting state
  codex ritual run shadow_integration # 3. Execute transformation
  codex reflect                       # 4. Seek wisdom
  codex state view                    # 5. Observe changes
"#;

    println!("{}", examples.bright_white());
}

// Helper function to handle graceful shutdown
pub fn handle_interrupt() {
    println!(
        "\n\n{}",
        "🌙 The ritual is paused. The sacred work awaits your return.".bright_blue()
    );
    std::process::exit(0);
}
