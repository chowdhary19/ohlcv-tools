# ohlcv-tools

Small CLI toolkit for working with OHLCV/tick market data.

## Subcommands

- `vwap` — volume-weighted average price from a trade/tick CSV
- `sma` / `ema` — simple/exponential moving averages over a price series
- `returns` — simple percentage returns between consecutive prices
- `drawdown` — max peak-to-trough drawdown over a price series
- `stats` — min/max/mean/median for a price series
- `correlation` — Pearson correlation between two equal-length price series
- `aggregate` — turn tick/trade data into OHLCV candles at a given interval
- `resample` — re-bucket already-aggregated OHLCV candles into a larger interval
- `completions` — generate a shell completion script

Every subcommand above (except `completions`) supports `--format json` for
scripting; see [JSON output](#json-output).

## Usage

```
cargo run -- info
cargo run -- vwap trades.csv
cargo run -- sma prices.csv --window 14
cargo run -- ema prices.csv --period 14
cargo run -- returns prices.csv
cargo run -- drawdown prices.csv
cargo run -- stats prices.csv --format json
cargo run -- correlation a.csv b.csv
cargo run -- aggregate ticks.csv --interval 60
cargo run -- resample candles.csv --interval 300
cargo run -- completions bash > ohlcv-tools.bash
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

`candles.csv` (for `resample`) must have `timestamp,open,high,low,close,volume`
columns — the same shape `aggregate` produces. Candles don't need to be
pre-sorted. Within each new, larger bucket: `open` comes from the
earliest-timestamped candle, `close` from the latest, `high`/`low` are the
extremes across all candles in the bucket, and `volume` is summed. This lets
you build higher-timeframe candles (e.g. 1m to 1h) without re-reading the
original ticks.

All numeric output is rounded via `--precision` (default 6 decimal places).

Every subcommand accepts `-` in place of a file path to read CSV from
stdin, e.g. `cat trades.csv | ohlcv-tools vwap -`.

## JSON output

Every computation subcommand accepts `--format json` for scripting instead
of the default plain-text output:

- Single-value results (`vwap`, `drawdown`, `correlation`) print a named
  JSON object, e.g. `{"vwap":17.5}`, `{"max_drawdown":0.25}`,
  `{"correlation":1.0}`.
- Series results (`sma`, `ema`, `returns`) print a JSON array of numbers,
  e.g. `[1.5,2.5,3.5]`.
- `aggregate` prints a JSON array of candle objects, e.g.
  `[{"timestamp":0,"open":100.0,"high":101.0,"low":100.0,"close":101.0,"volume":3.0}]`.
- `stats` prints a JSON object with `min`/`max`/`mean`/`median` keys.

```
cargo run -- vwap trades.csv --format json
cargo run -- sma prices.csv --window 14 --format json
cargo run -- aggregate ticks.csv --interval 60 --format json
```

## Development

```
cargo build
cargo test
```

Tests include unit tests per module plus `tests/cli.rs`, which exercises
the compiled binary end-to-end (argument parsing, exit codes, real
stdout/stderr output).
