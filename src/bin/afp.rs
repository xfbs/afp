extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use std::env;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let area: gtk::Notebook = gtk::Notebook::new();

    window.set_title("Amateurfunkprüfung");
    
    let button = gtk::Button::new_with_label("Übersicht");
    let label = gtk::Label::new("Übersicht");
    area.append_page(&button, Some(&label));

    let button = gtk::Button::new_with_label("Technik E");
    let label = gtk::Label::new("Technik E");
    area.append_page(&button, Some(&label));

    let button = gtk::Button::new_with_label("Technik A");
    let label = gtk::Label::new("Technik A");
    area.append_page(&button, Some(&label));

    let button = gtk::Button::new_with_label("Betrieb & Vorschriften");
    let label = gtk::Label::new("Betrieb & Vorschriften");
    area.append_page(&button, Some(&label));

    window.add(&area);
    window.set_default_size(500, 400);
    window.set_position(gtk::WindowPosition::Center);
    window.show_all();

    button.connect_clicked(|_| {
        println!("Clicked!");
    });
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
