#[macro_use]
extern crate clap;

extern crate calamine;
extern crate csv;

mod error;
mod table;

use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;

use error::Error;
use table::Table;

const DEFAULT_INPUT_PATH: &str = "input.xls";
const DEFAULT_SHEET_NAME: &str = "Sheet1";
const DEFAULT_CHUNK_COUNT: u64 = 5;

struct Settings {
    path: Option<PathBuf>,
    sheet: Option<String>,
    chunks: Option<u64>,
    max: Option<u64>,
}

impl Settings {
    fn from(args: &clap::ArgMatches) -> Settings {
        Settings {
            path: args.value_of("input").map(PathBuf::from),
            sheet: args.value_of("sheet").map(String::from),
            chunks: args.value_of("chunks").and_then(|n| {
                Settings::num_or_default_with_msg(n, "invalid number for chunks, using default")
            }),
            max: args.value_of("max").and_then(|n| {
                Settings::num_or_default_with_msg(n, "invalid number for max, using default")
            }),
        }
    }
    fn num_or_default_with_msg(arg: &str, msg: &str) -> Option<u64> {
        match FromStr::from_str(arg) {
            Err(_) => {
                eprintln!("{}", msg);
                None
            }
            Ok(n) => Some(n),
        }
    }
}

fn main() {
    let app = clap::App::new("splitter")
        .version(crate_version!())
        .about("split excel files into chunks")
        .author(crate_authors!())
        .args(&[
            clap::Arg::with_name("input")
                .help("The input excel file")
                .index(1),
            clap::Arg::with_name("sheet")
                .help("The name of the sheet to split")
                .long("sheet")
                .short("s")
                .takes_value(true),
            clap::Arg::with_name("chunks")
                .help("The number of chunks to produce")
                .long("chunks")
                .short("c")
                .takes_value(true),
            clap::Arg::with_name("max")
                .help("Maximum rows per chunk")
                .long("max")
                .short("m")
                .takes_value(true),
        ]);

    let matches = app.get_matches();
    let settings = Settings::from(&matches);

    match command_line(settings) {
        Ok(()) => (),
        Err(ref e) => {
            eprintln!("error: {}", e);
            ::std::process::exit(-1);
        }
    }
}

fn command_line(settings: Settings) -> Result<(), Error> {
    // Destruct Settings, loading defaults.
    let path = settings
        .path
        .unwrap_or_else(|| PathBuf::from(DEFAULT_INPUT_PATH));
    let sheet = settings
        .sheet
        .unwrap_or_else(|| DEFAULT_SHEET_NAME.to_owned());
    let chunks = settings.chunks.unwrap_or(DEFAULT_CHUNK_COUNT);
    let max = settings.max;

    // Load the tables
    let input = Table::read_excel(&path, &sheet)?;
    let (tables, rem) = input.split(chunks, max);

    for (i, table) in tables.iter().enumerate() {
        let out_path = path.with_file_name(format!(
            "{}-part-{}.csv",
            path.file_stem()
                .unwrap_or_else(|| OsStr::new("output"))
                .to_string_lossy(),
            i + 1
        ));

        table.write_csv(out_path)?;
        println!("Wrote {} rows to part {}.", table.body.len(), i+1);
    }

    if let Some(rows) = rem {
        let row_count = rows.len();
        let rem_table = Table {
            header: input.header.clone(),
            body: rows,
        };
        let rem_path =  path.with_file_name(format!(
            "{}-remainder.csv",
            path.file_stem()
                .unwrap_or_else(|| OsStr::new("output"))
                .to_string_lossy(),
        ));
        rem_table.write_csv(rem_path)?;
        println!("Wrote {} rows to the remainder file.", row_count);
    }

    println!("Done!");
    Ok(())
}
