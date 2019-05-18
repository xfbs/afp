extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::rc::Rc;
use crate::*;
use crate::ui::*;
use std::env;
use std::cell::RefCell;

/// CSS style for this app.
const STYLE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/style.css"));

#[derive(Clone)]
pub struct App {
    app: gtk::Application,
    main: MainView,
    main_controller: MainController,
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            app: gtk::Application::new(name, gio::ApplicationFlags::FLAGS_NONE).expect("application startup failed"),
            main: MainView::new(),
            main_controller: MainController::new(),
        }
    }

    fn startup(&self) {
        self.setup_accels();
        self.load_css();
    }

    fn setup_accels(&self) {
        self.app.set_accels_for_action("app.quit", &["<Primary>Q"]);
    }

    fn load_css(&self) {
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

    fn shutdown(&self) {
        // TODO save state?
    }

    fn activate(&self) {
        let app = &self.app;
        let mainview = &mut self.main.clone();
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
        let datastore = Rc::new(RefCell::new(datastore));
        mainview.init(datastore.clone());

        // position window and make visible
        window.add(&mainview.area);
        window.set_default_size(500, 400);
        window.set_position(gtk::WindowPosition::Center);
        window.show_all();
    }

    pub fn init(&self) {
        let app = self.clone();
        self.app.connect_startup(move |_| app.startup());
        let app = self.clone();
        self.app.connect_shutdown(move |_| {app.shutdown();});
        let app = self.clone();
        self.app.connect_activate(move |_| app.activate());
    }

    pub fn run(&self) {
        self.app.run(&env::args().collect::<Vec<_>>());
    }
}

