extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::rc::Rc;
use crate::*;
use crate::ui::*;
use std::env;
use std::sync::Mutex;

/// CSS style for this app.
const STYLE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/style.css"));

#[derive(Debug, Clone)]
pub struct App {
    app: gtk::Application,
    main: MainView
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            app: gtk::Application::new(name, gio::ApplicationFlags::FLAGS_NONE).expect("application startup failed"),
            main: MainView::new()
        }
    }

    fn startup(app: &gtk::Application) {
        app.set_accels_for_action("app.quit", &["<Primary>Q"]);

        // load css
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn shutdown(_app: &gtk::Application) {
        // TODO save state?
    }

    fn activate(app: &gtk::Application, mainview: &MainView) {
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
        mainview.init(&datastore);
        let datastore = Rc::new(Mutex::new(datastore));

        for (_i, section) in mainview.sections.borrow().iter().enumerate() {
            let mainview = mainview.clone();
            let datastore = datastore.clone();

            section.exam.connect_clicked(move |_widget| {
                datastore.lock().unwrap().section_mut(0).unwrap().question_mut(0).unwrap().answer(0);
                mainview.overview.update(&datastore.lock().unwrap());
            });
        }

        // position window and make visible
        window.add(&mainview.area);
        window.set_default_size(500, 400);
        window.set_position(gtk::WindowPosition::Center);
        window.show_all();
    }

    pub fn init(&self) {
        self.app.connect_startup(Self::startup);
        self.app.connect_shutdown(Self::shutdown);
        let mainview = self.main.clone();
        self.app.connect_activate(move |app| Self::activate(app, &mainview));
    }

    pub fn run(&self) {
        self.app.run(&env::args().collect::<Vec<_>>());
    }
}

