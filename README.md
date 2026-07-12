# ohlcv-tools

Small CLI toolkit for working with OHLCV/tick market data.

## Subcommands

- `vwap` — volume-weighted average price from a trade/tick CSV
- `sma` / `ema` — simple/exponential moving averages over a price series
- `returns` — simple percentage returns between consecutive prices
- `drawdown` — max peak-to-trough drawdown over a price series
- `aggregate` — turn tick/trade data into OHLCV candles at a given interval

## Usage

```
cargo run -- info
cargo run -- vwap trades.csv
cargo run -- sma prices.csv --window 14
cargo run -- ema prices.csv --period 14
cargo run -- returns prices.csv
cargo run -- drawdown prices.csv
cargo run -- aggregate ticks.csv --interval 60
```

`trades.csv` must have `price` and `volume` columns:

```
price,volume
100,10
101,5
99,20
```

`prices.csv` (for `sma`/`ema`) must have a `price` column:

```
price
100
101
99
```

`ticks.csv` (for `aggregate`) must have `timestamp` (unix seconds), `price`,
and `volume` columns. Ticks don't need to be pre-sorted; output is one
candle per line: `timestamp,open,high,low,close,volume`.

All numeric output is rounded via `--precision` (default 6 decimal places).

Every subcommand accepts `-` in place of a file path to read CSV from
stdin, e.g. `cat trades.csv | ohlcv-tools vwap -`.

## Development

```
cargo build
cargo test
```

Tests include unit tests per module plus `tests/cli.rs`, which exercises
the compiled binary end-to-end (argument parsing, exit codes, real
stdout/stderr output).
