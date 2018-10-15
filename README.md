# Splitter

Split an excel file into parts.

This is a pretty specific, single-purpose utility I made for work.

## Features

- [x] Split a CSV file into a number of smaller files by rows.
- [x] Include the header in each file.
- [x] Pick a number of files easily.
- [x] Set a max number of rows for the files
  - [x] Move extra rows into a remainder file.
- [ ] Pick columns to include.

## Installation

Grab the `.exe` file from the [releases][] page and move it to a safe place.

[releases]: https://github.com/isaacazuelos/splitter/releases

## Use

Make sure it's in your `PATH` if you're not running it specifically. Use the built-in help for basic guidance.

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

Run tests with `cargo test`.