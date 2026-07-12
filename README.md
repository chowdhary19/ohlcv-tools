# ohlcv-tools

Small CLI toolkit for working with OHLCV/tick market data.

## Subcommands

- `vwap` — volume-weighted average price from a trade/tick CSV
- `sma` / `ema` (planned) — simple/exponential moving averages over a price series
- `aggregate` (planned) — turn tick/trade data into OHLCV candles at a given interval

## Usage

```
cargo run -- info
cargo run -- vwap trades.csv
```

`trades.csv` must have `price` and `volume` columns:

```
price,volume
100,10
101,5
99,20
```

## Development

```
cargo build
cargo test
```
