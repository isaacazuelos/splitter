use calamine as xl;
use calamine::*;
use csv;

use std::ffi::OsStr;
use std::iter::repeat;
use std::path::Path;

use error::Error;

pub type Row = Vec<String>;

#[derive(Debug, Clone)]
pub struct Table {
    pub header: Row,
    pub body: Vec<Row>,
}

impl Table {
    pub fn read_excel(path: impl AsRef<Path>, sheet: &str) -> Result<Table, Error> {
        let path = path.as_ref();
        if !Table::valid_ext(path) {
            return Err(Error::InvalidExtension);
        }

        let mut data = xl::open_workbook_auto(path)?;
        let range = data.worksheet_range(sheet)
            .ok_or_else(|| Error::SheetMissing(sheet.to_owned()))??;

        let mut all_rows = Vec::new();
        for row_cells in range.rows() {
            let mut row = Vec::new();
            for cell in row_cells.iter() {
                row.push(value_to_string(cell))
            }
            all_rows.push(row);
        }

        let header = all_rows.get(0).cloned().unwrap_or_default();
        let body = all_rows[1..].to_vec();

        Ok(Table { header, body })
    }
    pub fn write_csv(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut w = csv::WriterBuilder::new().terminator(csv::Terminator::CRLF).from_path(path.as_ref())?;

        w.write_record(&self.header)?;
        for row in &self.body {
            w.write_record(row)?;
        }

        w.flush()?;

        Ok(())
    }
    pub fn split(&self, count: u64, max_out: Option<u64>) -> (Vec<Table>, Option<Vec<Row>>) {
        let mut rem = max_out.map(|_| Vec::new());
        let mut tables: Vec<Table> = repeat(Table {
            body: Vec::new(),
            header: self.header.clone(),
        }).take(count as usize)
            .collect();

        for (i, row) in self.body.iter().enumerate() {
            if let Some(max) = max_out {
                if i < (max * count) as usize {
                    tables[i % count as usize].body.push(row.clone());
                } else {
                    match rem {
                        None => (),
                        Some(ref mut v) => v.push(row.clone()),
                    }
                }
            } else {
                tables[i % count as usize].body.push(row.clone());
            }
        }

        (tables, rem)
    }

    fn valid_ext(path: &Path) -> bool {
        match path.extension().and_then(OsStr::to_str) {
            Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => true,
            _ => false,
        }
    }
}

fn value_to_string(v: &xl::DataType) -> String {
    match v {
        DataType::Empty => String::new(),
        DataType::String(ref s) => s.to_owned(),
        DataType::Float(ref f) => format!("{}", f),
        DataType::Int(ref i) => format!("{}", i),
        DataType::Error(ref e) => format!("{:?}", e),
        DataType::Bool(ref b) => format!("{}", b),
    }
}
