use clap::{Parser, Subcommand};

/// Small CLI toolkit for OHLCV/tick market data.
#[derive(Parser)]
#[command(name = "ohlcv-tools", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Print toolkit info and available subcommands.
    Info,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Info) | None => {
            println!("ohlcv-tools: VWAP, moving averages, and candle aggregation for market data.");
            println!("Run `ohlcv-tools --help` to see available subcommands as they're added.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
