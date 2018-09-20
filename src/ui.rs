//! UI definitions.
//!
//! | Label with drop instructions | <- Just a label
//! | Text Area                    | <- Drag and drop area
//! | Label for people slider      | <- Label
//! | People slider                | <- Scale from 0 to 20 by 1
//! | Label for rows per person    | <- Label
//! | Rows slider                  | <- Scale from 0 to 500 by 10
//1 | Process button               | <- Button

use gdk::DragAction;
use gtk;
use gtk::prelude::*;
use gtk::{DestDefaults, TargetFlags};
use url::Url;

fn drop_instructions() -> gtk::Label {
    gtk::Label::new("Drag files and/or folders onto the TextView below.")
}

fn text_area() -> gtk::ScrolledWindow {
    // Create scrollable text view as our drag target
    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_cursor_visible(false);
    let scrolled_text_view = gtk::ScrolledWindow::new(None, None);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);

    // Configure the text view to accept URI lists from other applications. This allows
    // dragging files & folders from a file browser program onto the textview.
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        TargetFlags::OTHER_APP,
        0,
    )];
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
    scrolled_text_view
}
fn split_instructions() -> gtk::Label {
    gtk::Label::new("How many files should the source files be split into?")
}
fn split_slider() -> gtk::Scale {
    gtk::Scale::new_with_range(gtk::Orientation::Horizontal, 0.0, 20.0, 1.0)
}
fn max_instructions() -> gtk::Label {
    gtk::Label::new("How many rows per split?")
}
fn max_slider() -> gtk::Scale {
    gtk::Scale::new_with_range(gtk::Orientation::Horizontal, 0.0, 500.0, 10.0)
}
fn process_btn() -> gtk::Button {
    gtk::Button::new_with_label("Process")
}

pub fn build(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Drag and Drop Example with a TextView");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    vbox.pack_start(&drop_instructions(), true, true, 0);
    vbox.pack_start(&text_area(), true, true, 0);
    vbox.pack_start(&split_instructions(), true, true, 0);
    vbox.pack_start(&split_slider(), true, true, 0);
    vbox.pack_start(&max_instructions(), true, true, 0);
    vbox.pack_start(&max_slider(), true, true, 0);
    vbox.pack_start(&process_btn(), true, true, 0);

    // Create a new window
    window.add(&vbox);
    window.show_all();

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });
}
