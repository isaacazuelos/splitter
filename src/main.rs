extern crate calamine;
extern crate csv;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate url;

use std::env;

mod error;
mod ui;
mod table;

use error::Error;
use table::Table;

use gio::prelude::*;

const NUMBER_OF_OUT_FILES: usize = 3;
const MAX_PER_OUT_FILE: Option<usize> = Some(100);
const INPUT_FILE_NAME: &str = "input.xls";
const REMAINDER_FILE_NAME: &str = "remainder.csv";

/// https://gtk-rs.org/docs/gtk/struct.CheckButton.html
/// https://developer.gnome.org/gtk3/stable/GtkCheckButton.html
/// https://github.com/gtk-rs/examples/blob/master/src/bin/basic.rs

fn main() -> Result<(), Box<::std::error::Error>> {
    let application =
        gtk::Application::new("com.github.drag_and_drop", gio::ApplicationFlags::empty())?;

    application.connect_startup(ui::build);
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());

    Ok(())
}

fn _process() -> Result<(), Error> {
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
