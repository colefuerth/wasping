# wasping

A concurrent ping engine in Rust. Used Ryan Prairie's [flytrap](https://github.com/prairir/flytrap) architecture.

*Requires `sudo` to run.*

## Usage

```bash
wasping <optional:-o|--output <output>> {optional:-l|--limit <limit>}
```

- output: The output file to write to. Defaults to `stdout`.
- limit: The maximum number of concurrent pings to run. Defaults to `10000`.
