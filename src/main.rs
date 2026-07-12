mod candles;
mod correlation;
mod drawdown;
mod format;
mod input;
mod moving_average;
mod returns;
mod stats;
mod vwap;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

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
        /// Path to a CSV file with `price` and `volume` columns, or `-` for stdin.
        file: PathBuf,
        /// Decimal places to round the output to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
    },
    /// Simple moving average over a `price` CSV column.
    Sma {
        /// Path to a CSV file with a `price` column, or `-` for stdin.
        file: PathBuf,
        /// Window size.
        #[arg(long, default_value_t = 14)]
        window: usize,
        /// Decimal places to round the output to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
    },
    /// Exponential moving average over a `price` CSV column.
    Ema {
        /// Path to a CSV file with a `price` column, or `-` for stdin.
        file: PathBuf,
        /// Smoothing period.
        #[arg(long, default_value_t = 14)]
        period: usize,
        /// Decimal places to round the output to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
    },
    /// Simple percentage returns between consecutive prices.
    Returns {
        /// Path to a CSV file with a `price` column, or `-` for stdin.
        file: PathBuf,
        /// Decimal places to round the output to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
    },
    /// Min/max/mean/median summary of a price series.
    Stats {
        /// Path to a CSV file with a `price` column, or `-` for stdin.
        file: PathBuf,
        /// Decimal places to round the output to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
        /// Output format.
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Maximum peak-to-trough drawdown over a price series.
    Drawdown {
        /// Path to a CSV file with a `price` column, or `-` for stdin.
        file: PathBuf,
        /// Decimal places to round the output to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
    },
    /// Pearson correlation between two equal-length price series.
    Correlation {
        /// Path to the first CSV file with a `price` column, or `-` for stdin.
        file_a: PathBuf,
        /// Path to the second CSV file with a `price` column.
        file_b: PathBuf,
        /// Decimal places to round the output to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
    },
    /// Generate a shell completion script.
    Completions {
        /// Shell to generate completions for.
        shell: Shell,
    },
    /// Aggregate tick data into OHLCV candles.
    Aggregate {
        /// Path to a CSV file with `timestamp`, `price`, and `volume` columns, or `-` for stdin.
        file: PathBuf,
        /// Candle interval in seconds.
        #[arg(long)]
        interval: i64,
        /// Decimal places to round OHLCV values to.
        #[arg(long, default_value_t = 6)]
        precision: u32,
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
        Some(Command::Vwap { file, precision }) => match vwap::load_trades(&file) {
            Ok(trades) => match vwap::vwap(&trades) {
                Some(value) => {
                    println!("{}", format::round_to(value, precision));
                    ExitCode::SUCCESS
                }
                None => {
                    eprintln!(
                        "ohlcv-tools: no trades or zero total volume in {}",
                        file.display()
                    );
                    ExitCode::FAILURE
                }
            },
            Err(e) => {
                eprintln!("ohlcv-tools: failed to read {}: {e}", file.display());
                ExitCode::FAILURE
            }
        },
        Some(Command::Drawdown { file, precision }) => match moving_average::load_prices(&file) {
            Ok(prices) => match drawdown::max_drawdown(&prices) {
                Some(value) => {
                    println!("{}", format::round_to(value, precision));
                    ExitCode::SUCCESS
                }
                None => {
                    eprintln!(
                        "ohlcv-tools: need at least 2 prices in {} to compute drawdown",
                        file.display()
                    );
                    ExitCode::FAILURE
                }
            },
            Err(e) => {
                eprintln!("ohlcv-tools: failed to read {}: {e}", file.display());
                ExitCode::FAILURE
            }
        },
        Some(Command::Stats {
            file,
            precision,
            format,
        }) => match moving_average::load_prices(&file) {
            Ok(prices) => match stats::summarize(&prices) {
                Some(s) => {
                    let rounded = stats::Stats {
                        min: format::round_to(s.min, precision),
                        max: format::round_to(s.max, precision),
                        mean: format::round_to(s.mean, precision),
                        median: format::round_to(s.median, precision),
                    };
                    match format {
                        OutputFormat::Text => {
                            println!("min: {}", rounded.min);
                            println!("max: {}", rounded.max);
                            println!("mean: {}", rounded.mean);
                            println!("median: {}", rounded.median);
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string(&rounded).unwrap());
                        }
                    }
                    ExitCode::SUCCESS
                }
                None => {
                    eprintln!("ohlcv-tools: no prices in {}", file.display());
                    ExitCode::FAILURE
                }
            },
            Err(e) => {
                eprintln!("ohlcv-tools: failed to read {}: {e}", file.display());
                ExitCode::FAILURE
            }
        },
        Some(Command::Correlation {
            file_a,
            file_b,
            precision,
        }) => {
            let prices_a = moving_average::load_prices(&file_a);
            let prices_b = moving_average::load_prices(&file_b);
            match (prices_a, prices_b) {
                (Ok(a), Ok(b)) => match correlation::pearson(&a, &b) {
                    Some(r) => {
                        println!("{}", format::round_to(r, precision));
                        ExitCode::SUCCESS
                    }
                    None => {
                        eprintln!(
                            "ohlcv-tools: correlation undefined (mismatched lengths, <2 points, or zero variance)"
                        );
                        ExitCode::FAILURE
                    }
                },
                (Err(e), _) => {
                    eprintln!("ohlcv-tools: failed to read {}: {e}", file_a.display());
                    ExitCode::FAILURE
                }
                (_, Err(e)) => {
                    eprintln!("ohlcv-tools: failed to read {}: {e}", file_b.display());
                    ExitCode::FAILURE
                }
            }
        }
        Some(Command::Completions { shell }) => {
            generate(
                shell,
                &mut Cli::command(),
                "ohlcv-tools",
                &mut std::io::stdout(),
            );
            ExitCode::SUCCESS
        }
        Some(Command::Sma {
            file,
            window,
            precision,
        }) => print_price_series(&file, window, precision, moving_average::sma),
        Some(Command::Ema {
            file,
            period,
            precision,
        }) => print_price_series(&file, period, precision, moving_average::ema),
        Some(Command::Returns { file, precision }) => match moving_average::load_prices(&file) {
            Ok(prices) => {
                let series = returns::simple_returns(&prices);
                if series.is_empty() {
                    eprintln!(
                        "ohlcv-tools: need at least 2 prices in {} to compute returns",
                        file.display()
                    );
                    return ExitCode::FAILURE;
                }
                for value in series {
                    println!("{}", format::round_to(value, precision));
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("ohlcv-tools: failed to read {}: {e}", file.display());
                ExitCode::FAILURE
            }
        },
        Some(Command::Aggregate {
            file,
            interval,
            precision,
        }) => match candles::load_ticks(&file) {
            Ok(ticks) => {
                let result = candles::aggregate(&ticks, interval);
                if result.is_empty() {
                    eprintln!(
                        "ohlcv-tools: no candles produced from {} (empty input or non-positive interval)",
                        file.display()
                    );
                    return ExitCode::FAILURE;
                }
                println!("timestamp,open,high,low,close,volume");
                for c in result {
                    println!(
                        "{},{},{},{},{},{}",
                        c.timestamp,
                        format::round_to(c.open, precision),
                        format::round_to(c.high, precision),
                        format::round_to(c.low, precision),
                        format::round_to(c.close, precision),
                        format::round_to(c.volume, precision),
                    );
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("ohlcv-tools: failed to read {}: {e}", file.display());
                ExitCode::FAILURE
            }
        },
    }
}

fn print_price_series(
    file: &std::path::Path,
    param: usize,
    precision: u32,
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
                println!("{}", format::round_to(value, precision));
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

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
