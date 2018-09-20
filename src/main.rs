extern crate calamine;
extern crate csv;

mod table;
mod error;

use table::{Table};

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