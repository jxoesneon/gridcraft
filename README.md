# GridCraft Studio

An open-source, sovereign spreadsheet and data modeling application built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![GridCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode spreadsheet UI with virtualized 1,000,000-row cell grid, formula bar, and emerald highlights.
- **`crates/engine`**: Dependency DAG calculation engine, formula evaluation parser, and high-performance XLSX reader/writer.

## Legal & Compliance Notice

GridCraft is an independent open-source spreadsheet application. It is not affiliated with Microsoft Corporation. Microsoft, Excel, and Office are trademarks of Microsoft Corporation. Alphanumeric grid coordinate systems (A1..Z100) and formula bar interfaces originate from VisiCalc (1979) and are public domain.

## License

Dual-licensed under MIT OR Apache-2.0.
