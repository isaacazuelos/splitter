extern crate calamine;
extern crate csv;

use std::path::Path;

const NUMBER_OF_OUT_FILES: usize = 3;
const MAX_PER_OUT_FILE: Option<usize> = Some(100);

const INPUT_FILE_NAME: &str = "input.xls";

const REMAINDER_FILE_NAME: &str = "remainder.csv";

fn main() -> Result<(), Box<::std::error::Error>> {
    let input = Table::read_excel(INPUT_FILE_NAME)?;

    let (tables, rem) = input.split(NUMBER_OF_OUT_FILES, MAX_PER_OUT_FILE);

    for (i, table) in tables.iter().enumerate() {
        table.write_csv(format!("part_{}.csv", i))?;
    }

    if let Some(rows) = rem {
        let rem_table = Table {
            header: input.header.clone(),
            body: rows,
        };
        rem_table.write_csv(REMAINDER_FILE_NAME)?;

    }

    Ok(())
}

type Row = Vec<String>;

#[derive(Debug, Clone)]
struct Table {
    header: Row,
    body: Vec<Row>,
}

impl Table {
    fn read_excel(_path: impl AsRef<Path>) -> Result<Table, Error> {
        unimplemented!();
    }
    fn write_csv(&self, _path: impl AsRef<Path>) -> Result<Table, Error> {
        unimplemented!()
    }
    fn split(&self, _count: usize, _max_out: Option<usize>) -> (Vec<Table>, Option<Vec<Row>>) {
        unimplemented!()
    }
}

#[derive(Debug)]
enum Error {
    IO(::std::io::Error),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::IO(ref e) => write!(f, "io error: {:?}", e),
        }
    }
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        match self {
            Error::IO(ref e) => Some(e),
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::IO(err)
    }
}