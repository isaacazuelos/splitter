# Splitter

Split an excel file into parts.

This is a pretty specific, single-purpose utility I made for work.

## Features

- [x] Split a CSV file into a number of smaller files by rows.
- [x] Include the header in each file.
- [x] Pick a number of files easily.
- [x] Set a max number of rows for the files
  - [x] move extra rows into a remainder file.
- [ ] Pick columns to include.

## Installation

Grab the `.exe` file from the [releases][] page and move it to a safe place.

[releases]: https://github.com/isaacazuelos/splitter/releases

## Running

Make sure it's in your `PATH` if you're not running it specifically. Use the built-in help for basic guidence.

Typical usage would look like:
```sh
$ splitter "Some Excel Table.xlsx" --chunks 5 --max 120 --sheet "Data"
Done! There are 123 non-header rows in "Some Excel Table_remainder.csv"
```

## Building

You can build the project with `cargo`. You'll need [Rust] installed.

[Rust]: https://rust-lang.org

Run tests with `cargo test`.