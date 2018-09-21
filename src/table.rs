use calamine as xl;
use calamine::*;

use std::ffi::OsStr;
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
        let range = data
            .worksheet_range(sheet)
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
    pub fn write_csv(&self, _path: impl AsRef<Path>) -> Result<Table, Error> {
        unimplemented!("Table::write_csv not implemented")
    }
    pub fn split(&self, _count: u64, _max_out: Option<u64>) -> (Vec<Table>, Option<Vec<Row>>) {
        unimplemented!("Table::split not implementd")
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
