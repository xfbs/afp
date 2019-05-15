extern crate afp;
extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use afp::*;

use std::env;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Amateurfunkprüfung");

    // load notebook view and overview tab
    let area = gtk::Notebook::new();
    let button = gtk::Button::new_with_label("Übersicht");
    let label = gtk::Label::new("Übersicht");
    area.append_page(&button, Some(&label));

    // load other tabs from data store
    let datastore = DataStore::load("");
    for section in datastore.sections() {
        let button = gtk::Button::new_with_label(section.short());
        let label = gtk::Label::new(section.short());
        area.append_page(&button, Some(&label));
    }

    // position window and make visible
    window.add(&area);
    window.set_default_size(500, 400);
    window.set_position(gtk::WindowPosition::Center);
    window.show_all();

    /*
    button.connect_clicked(|_| {
        println!("Clicked!");
    });
    */
}

fn main() {
    let uiapp = gtk::Application::new("net.xfbs.afs",
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");

    uiapp.connect_activate(|app| {
        build_ui(app);
    });

    uiapp.run(&env::args().collect::<Vec<_>>());
}
