# ohlcv-tools

Small CLI toolkit for working with OHLCV/tick market data.

## Subcommands

- `vwap` — volume-weighted average price from a trade/tick CSV
- `sma` / `ema` — simple/exponential moving averages over a price series
- `aggregate` (planned) — turn tick/trade data into OHLCV candles at a given interval

## Usage

```
cargo run -- info
cargo run -- vwap trades.csv
cargo run -- sma prices.csv --window 14
cargo run -- ema prices.csv --period 14
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

## Development

```
cargo build
cargo test
```
