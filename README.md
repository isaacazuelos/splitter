# Splitter

Split an excel file into CSV chunks.

This is a pretty specific, single-purpose utility I made for work. You probably don't want it.

## Installation

Grab the `.exe` file from the [releases][] page and move it to a safe place.

[releases]: https://github.com/isaacazuelos/splitter/releases

## Use

Make sure it's in your `PATH` if you're not running it from it's full path. Use the built-in `--help` for basic guidance.

Typical usage would look like:

``` sh
$ splitter "Some Excel Table.xlsx" --chunks 1 --max 120 --sheet "Data"
Wrote 120 rows to part 1.
Wrote 143 rows to the remainder file.
Done!
```

## Building

You can build the project with `cargo`. You'll need [Rust] installed.

[Rust]: https://rust-lang.org

If there were tests, you could run them with `cargo test`.

## License

This project is under the MIT license.

See the included LICENSE file.
