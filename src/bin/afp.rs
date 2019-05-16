extern crate afp;
extern crate gtk;
extern crate gio;
extern crate pango;
extern crate cairo;

use gtk::prelude::*;
use gio::prelude::*;
use pango::prelude::*;
use cairo::prelude::*;
use afp::*;
use std::env;
use std::ops::Deref;

struct MainView {
    area: gtk::Notebook,
    overview: OverView,
    section_labels: Vec<gtk::Label>,
    section_bodies: Vec<gtk::Button>
}

struct OverView {
    body: gtk::Grid,
    label: gtk::Label,
    title: gtk::Label,
}

impl OverView {
    fn new(datastore: &DataStore) -> OverView {
        let body = gtk::Grid::new();
        body.set_margin_top(10);
        body.set_margin_bottom(10);
        body.set_margin_start(10);
        body.set_margin_end(10);
        body.set_column_spacing(10);
        body.set_row_spacing(10);
        let label = gtk::Label::new("Übersicht");
        let title = gtk::Label::new(None);
        title.set_markup("<span font-size=\"xx-large\" font-weight=\"heavy\">Amateurfunkprüfer</span>");
        title.set_hexpand(true);
        body.attach(&title, 0, 0, datastore.sections().len() as i32, 1);

        let ov = OverView {
            body: body,
            label: label,
            title: title
        };

        ov.update_status(datastore);

        ov
    }

    fn update_status(&self, datastore: &DataStore) {
        for (i, section) in datastore.sections().iter().enumerate() {
            let label = gtk::Label::new(None);
            label.set_markup(&format!("{} red, {} yellow, {} green, {} total",
                                     section.count_by_state(QuestionState::Red),
                                     section.count_by_state(QuestionState::Yellow),
                                     section.count_by_state(QuestionState::Green),
                                     section.count()));
            let count = section.count();
            let count_green = section.count_by_state(QuestionState::Green);
            let count_green_f = count_green as f64 / count as f64;
            let area = gtk::DrawingArea::new();
            area.set_size_request(100, 100);
            area.set_hexpand(true);
            area.connect_draw(move |_, surface| {
                let width = 100 as f64;
                let height = 100 as f64;
                surface.arc(width / 2.0, height / 2.0, width / 2.0, 0.0, 0.5 * 3.14);
                surface.set_source_rgba(0.0, 1.0, 0.0, 0.8);
                surface.fill();
                surface.arc(width / 2.0, height / 2.0, width / 2.0, 0.5 * 3.14, 1.0 * 3.14);
                surface.set_source_rgba(1.0, 1.0, 0.0, 0.8);
                surface.fill();

                Inhibit(false)
            });
            self.body.attach(&label, i as i32, 1, 1, 1);
            self.body.attach(&area, i as i32, 2, 1, 1);
        }
    }
}

impl MainView {
    fn new(datastore: &DataStore) -> MainView {
        let area = gtk::Notebook::new();
        let overview = OverView::new(datastore);

        area.append_page(&overview.body, Some(&overview.label));

        let mut mv = MainView {
            area: area,
            overview: overview,
            section_labels: Vec::new(),
            section_bodies: Vec::new()
        };

        for section in datastore.sections() {
            mv.add_section(section);
        }

        mv
    }

    fn add_section(&mut self, section: &Section) {
        let section_label = gtk::Label::new(section.short());
        let section_body = gtk::Button::new_with_label(section.short());
        self.area.append_page(&section_body, Some(&section_label));
    }
}

impl Deref for MainView {
    type Target = gtk::Notebook;

    fn deref(&self) -> &gtk::Notebook {
        &self.area
    }
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Amateurfunkprüfung");

    let datastore = DataStore::load(&std::path::PathBuf::from("/Users/pelsen/.config/afp/datastore.yml")).unwrap();
    let mainview = MainView::new(&datastore);

    // position window and make visible
    window.add(&mainview.area);
    window.set_default_size(500, 400);
    window.set_position(gtk::WindowPosition::Center);
    window.show_all();
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
