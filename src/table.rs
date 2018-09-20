use std::path::Path;

use error::Error;

pub type Row = Vec<String>;

#[derive(Debug, Clone)]
pub struct Table {
    pub header: Row,
    pub body: Vec<Row>,
}

impl Table {
    pub fn read_excel(_path: impl AsRef<Path>) -> Result<Table, Error> {
        unimplemented!();
    }
    pub fn write_csv(&self, _path: impl AsRef<Path>) -> Result<Table, Error> {
        unimplemented!()
    }
    pub fn split(&self, _count: usize, _max_out: Option<usize>) -> (Vec<Table>, Option<Vec<Row>>) {
        unimplemented!()
    }
}