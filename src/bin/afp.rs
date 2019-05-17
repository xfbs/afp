extern crate afp;
extern crate gtk;
extern crate gio;
//extern crate pango;
//extern crate cairo;

use gtk::prelude::*;
use gio::prelude::*;
//use pango::prelude::*;
//use cairo::prelude::*;
use afp::*;
use std::env;
use std::ops::Deref;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Mutex;
//use std::sync::Arc;

struct App {
    app: gtk::Application
}

#[derive(Debug, Clone)]
struct MainView {
    area: gtk::Notebook,
    overview: OverView,
    sections: Vec<SectionView>
}

#[derive(Debug, Clone)]
struct OverView {
    body: gtk::Grid,
    label: gtk::Label,
    title: gtk::Label,
    section_labels: Vec<gtk::Label>,
    section_charts: Vec<gtk::DrawingArea>,
}

#[derive(Debug, Clone)]
struct SectionView {
    label: gtk::Label,
    body: gtk::Grid,
    title: gtk::Label,
    button: gtk::Button,
}

impl OverView {
    fn new() -> OverView {
        let body = gtk::Grid::new();
        let label = gtk::Label::new("Übersicht");
        let title = gtk::Label::new(None);

        OverView {
            body: body,
            label: label,
            title: title,
            section_labels: Vec::new(),
            section_charts: Vec::new(),
        }
    }

    fn init(&mut self, datastore: &DataStore) {
        // this method may be called multiple times, so here we clean
        // out the trash
        self.body.foreach(|widget| {
            self.body.remove(widget);
        });

        self.section_labels.clear();
        self.section_charts.clear();

        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.title.set_markup("<span font-size=\"xx-large\" font-weight=\"heavy\">Übersicht</span>");
        self.title.set_hexpand(true);
        self.body.attach(&self.title, 0, 0, datastore.sections().len() as i32, 1);

        for (i, _section) in datastore.sections().iter().enumerate() {
            let label = gtk::Label::new(None);
            label.set_hexpand(true);
            self.body.attach(&label, i as i32, 1, 1, 1);
            self.section_labels.push(label);

            let area = gtk::DrawingArea::new();
            area.set_size_request(100, 100);
            area.set_hexpand(true);
            self.body.attach(&area, i as i32, 2, 1, 1);
            self.section_charts.push(area);
        }
    }

    fn update(&self, datastore: &DataStore) {
        for (i, section) in datastore.sections().iter().enumerate() {
            // title
            self.section_labels[i].set_text(section.name());

            let count = section.count();
            let count_green = section.count_by_state(QuestionState::Green);
            let count_yellow = section.count_by_state(QuestionState::Yellow);

            self.section_charts[i].connect_draw(move |widget, cairo| {
                let width = 100 as f64;
                let height = 100 as f64;
                let lwidth = 6.0;

                // make sure we're centered
                cairo.translate(widget.get_allocated_width() as f64 / 2.0 - width / 2.0, 0.0);

                // rotate by centerpoint of circle to get angles right
                cairo.translate(width / 2.0, height / 2.0);
                cairo.rotate(1.5 * PI);
                cairo.translate(-width / 2.0, -height / 2.0);
                cairo.set_line_width(lwidth);
                let draw_part = |cairo: &cairo::Context, start: f64, stop: f64| {
                    cairo.arc(width / 2.0 + 0.5 * lwidth,
                              height / 2.0,
                              width / 2.0 - lwidth, 
                              start * 2.0 * PI, 
                              stop * 2.0 *  PI);
                };

                if count > 0 {
                    let end_green = (count_green as f64) / (count as f64);
                    draw_part(cairo, 0.0, end_green);
                    cairo.set_source_rgba(0.20, 0.80, 0.20, 0.8);
                    cairo.stroke();
                    let end_yellow = end_green + (count_yellow as f64) / (count as f64);
                    draw_part(cairo, end_green, end_yellow);
                    cairo.set_source_rgba(0.80, 0.89, 0.20, 0.8);
                    cairo.stroke();
                    draw_part(cairo, end_yellow, 1.0);
                    cairo.set_source_rgba(0.80, 0.20, 0.20, 0.8);
                    cairo.stroke();
                } else {
                    draw_part(cairo, 0.0, 1.0);
                    cairo.set_source_rgba(0.80, 0.20, 0.20, 0.8);
                    cairo.stroke();
                }

                Inhibit(false)
            });
        }
    }
}

impl SectionView {
    fn new() -> SectionView {
        let label = gtk::Label::new(None);
        let body = gtk::Grid::new();
        let title = gtk::Label::new(None);
        let button = gtk::Button::new();

        SectionView {
            label: label,
            body: body,
            title: title,
            button: button
        }
    }

    fn init(&self, _section: &Section) {
        // cleanup
        self.body.foreach(|widget| {
            self.body.remove(widget);
        });

        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.title.set_hexpand(true);
        self.body.attach(&self.title, 0, 0, 1, 1);
        self.body.attach(&self.button, 0, 1, 1, 1);
    }

    fn update(&self, section: &Section) {
        self.label.set_text(section.short());
        self.title.set_markup(&format!("<span font-size=\"xx-large\" font-weight=\"heavy\">{}</span>", section.name()));
        self.button.set_label("Button.");
    }
}

impl MainView {
    fn new(datastore: &DataStore) -> MainView {
        let area = gtk::Notebook::new();
        let mut overview = OverView::new();
        overview.init(datastore);
        overview.update(datastore);

        area.append_page(&overview.body, Some(&overview.label));

        let mut mv = MainView {
            area: area,
            overview: overview,
            sections: Vec::new(),
        };

        for section in datastore.sections() {
            mv.add_section(section);
        }

        mv
    }

    fn add_section(&mut self, sec: &Section) {
        let mut section = SectionView::new();
        section.init(sec);
        section.update(sec);
        self.area.append_page(&section.body, Some(&section.label));
        self.sections.push(section);
    }
}

/*
impl Deref for MainView {
    type Target = gtk::Notebook;

    fn deref(&self) -> &gtk::Notebook {
        &self.area
    }
}
*/

impl App {
    fn new(name: &str) -> App {
        App {
            app: gtk::Application::new(name, gio::ApplicationFlags::FLAGS_NONE).expect("application startup failed")
        }
    }

    fn init(&self) {
        self.app.connect_startup(|app| {
            app.set_accels_for_action("app.quit", &["<Primary>Q"]);
        });

        self.app.connect_activate(|app| {
            let window = gtk::ApplicationWindow::new(app);
            window.set_title("Amateurfunkprüfung");

            // menu bar
            let menu = gio::Menu::new();
            let menu_bar = gio::Menu::new();

            // define quit action
            let quit = gio::SimpleAction::new("quit", None);
            let window_clone = window.clone();
            quit.connect_activate(move |_, _| {
                window_clone.destroy();
            });
            app.add_action(&quit);

            menu.append("Quit", "app.quit");
            app.set_app_menu(&menu);
            app.set_menubar(&menu_bar);

            let datastore = DataStore::load(&std::path::PathBuf::from("/Users/pelsen/.config/afp/datastore.yml")).unwrap();
            let mainview = MainView::new(&datastore);
            let datastore = Rc::new(Mutex::new(datastore));

            for (_i, section) in mainview.clone().sections.iter().enumerate() {
                let mainview = mainview.clone();
                let datastore = datastore.clone();

                section.button.connect_clicked(move |_widget| {
                    datastore.lock().unwrap().section_mut(0).unwrap().question_mut(0).unwrap().answer(0);
                    mainview.overview.update(&datastore.lock().unwrap());
                });
            }

            // position window and make visible
            window.add(&mainview.area);
            window.set_default_size(500, 400);
            window.set_position(gtk::WindowPosition::Center);
            window.show_all();
        });
    }

    fn run(&self) {
        self.app.run(&env::args().collect::<Vec<_>>());
    }
}

fn main() {
    let app = App::new("net.xfbs.afs");
    app.init();
    app.run();
}
