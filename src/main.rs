extern crate calamine;
extern crate csv;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate url;

use url::Url;

use std::env;

mod error;
mod table;

use error::Error;
use table::Table;

use gdk::DragAction;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{DestDefaults, TargetFlags};

const NUMBER_OF_OUT_FILES: usize = 3;
const MAX_PER_OUT_FILE: Option<usize> = Some(100);
const INPUT_FILE_NAME: &str = "input.xls";
const REMAINDER_FILE_NAME: &str = "remainder.csv";


//
// | Label with drop instructions | <- Just a label
// | Text Area                    | <- DnDable 
// | Label for people slider      | <- Label
// | People slider                | <- Scale from 0 to 20 by 1
// | Label for rows per person    | <- Label
// | Rows slider                  | <- Scale from 0 to 500 by 10
// | Process button               | <- Button
//
// Alert on success.
//
fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Drag and Drop Example with a TextView");

    // Give a nice text description for the user
    let label = gtk::Label::new("Drag files and/or folders onto the TextView below.");

    // Create scrollable text view as our drag target
    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_cursor_visible(false);
    let scrolled_text_view = gtk::ScrolledWindow::new(None, None);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);

    // Configure the text view to accept URI lists from other applications. This allows
    // dragging files & folders from a file browser program onto the textview.
    let targets = vec![gtk::TargetEntry::new("text/uri-list", TargetFlags::OTHER_APP, 0)];
    text_view.drag_dest_set(DestDefaults::HIGHLIGHT, &targets, DragAction::COPY);

    // Process any `drag-data-received` events received by the textview. These events include
    // the URL list we're looking for.
    text_view.connect_drag_data_received(|w, _, _, _, d, _, _| {
        // Get the text buffer for the TextView and clear it to make it ready to accept new text.
        let buffer = w.get_buffer().unwrap();
        buffer.set_text("");

        // Since we only accept `text/uri-list`s here, we don't need to check first, we can simply
        // iterate through all of the accepted URIs.
        for file in d.get_uris() {
            let file_path = Url::parse(&file).unwrap();
            let file_path = file_path.to_file_path().unwrap();
            let file_path_str = file_path.to_str().unwrap();
            let bulleted_file_path = format!(" • {}\n", &file_path_str);
            // We make sure to always insert this at the end of the text buffer so they're in
            // order.
            buffer.insert_at_cursor(&bulleted_file_path);
        }
    });

    let button = gtk::Button::new_with_label("Process");
    let check = gtk::CheckButton::new_with_label("Limit to 100 elements");
    let scale = gtk::Scale::new_with_range(gtk::Orientation::Horizontal, 0.0, 15.0, 1.0);

    // Pack widgets vertically.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&scrolled_text_view, true, true, 0);
    vbox.pack_start(&scale, true, true, 0);
    vbox.pack_start(&check, true, true, 0);
    vbox.pack_start(&button, true, true, 0);

    // Create a new window
    window.add(&vbox);
    window.show_all();

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });
}

fn main() -> Result<(), Box<::std::error::Error>> {
    let application =
        gtk::Application::new("com.github.drag_and_drop", gio::ApplicationFlags::empty())?;

    application.connect_startup(build_ui);
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
