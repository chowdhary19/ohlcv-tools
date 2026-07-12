mod moving_average;
mod vwap;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

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
    /// Compute volume-weighted average price from a `price,volume` CSV.
    Vwap {
        /// Path to a CSV file with `price` and `volume` columns.
        file: PathBuf,
    },
    /// Simple moving average over a `price` CSV column.
    Sma {
        /// Path to a CSV file with a `price` column.
        file: PathBuf,
        /// Window size.
        #[arg(long, default_value_t = 14)]
        window: usize,
    },
    /// Exponential moving average over a `price` CSV column.
    Ema {
        /// Path to a CSV file with a `price` column.
        file: PathBuf,
        /// Smoothing period.
        #[arg(long, default_value_t = 14)]
        period: usize,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Info) | None => {
            println!("ohlcv-tools: VWAP, moving averages, and candle aggregation for market data.");
            println!("Run `ohlcv-tools --help` to see available subcommands as they're added.");
            ExitCode::SUCCESS
        }
        Some(Command::Vwap { file }) => match vwap::load_trades(&file) {
            Ok(trades) => match vwap::vwap(&trades) {
                Some(value) => {
                    println!("{value}");
                    ExitCode::SUCCESS
                }
                None => {
                    eprintln!("ohlcv-tools: no trades or zero total volume in {}", file.display());
                    ExitCode::FAILURE
                }
            },
            Err(e) => {
                eprintln!("ohlcv-tools: failed to read {}: {e}", file.display());
                ExitCode::FAILURE
            }
        },
        Some(Command::Sma { file, window }) => {
            print_price_series(&file, window, moving_average::sma)
        }
        Some(Command::Ema { file, period }) => {
            print_price_series(&file, period, moving_average::ema)
        }
    }
}

fn print_price_series(
    file: &std::path::Path,
    param: usize,
    compute: fn(&[f64], usize) -> Vec<f64>,
) -> ExitCode {
    match moving_average::load_prices(file) {
        Ok(prices) => {
            let series = compute(&prices, param);
            if series.is_empty() {
                eprintln!(
                    "ohlcv-tools: not enough prices in {} for the requested window/period",
                    file.display()
                );
                return ExitCode::FAILURE;
            }
            for value in series {
                println!("{value}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ohlcv-tools: failed to read {}: {e}", file.display());
            ExitCode::FAILURE
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
