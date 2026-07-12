# ohlcv-tools

Small CLI toolkit for working with OHLCV/tick market data.

## Planned subcommands

- `vwap` — volume-weighted average price from a trade/tick CSV
- `sma` / `ema` — simple/exponential moving averages over a price series
- `aggregate` — turn tick/trade data into OHLCV candles at a given interval

## Usage

```
cargo run -- info
```

## Development

```
cargo build
cargo test
```
