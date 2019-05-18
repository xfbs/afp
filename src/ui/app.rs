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
    window: Rc<RefCell<Option<gtk::ApplicationWindow>>>,
    main: MainController,
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            app: gtk::Application::new(name, gio::ApplicationFlags::FLAGS_NONE).expect("application startup failed"),
            window: Rc::new(RefCell::new(None)),
            main: MainController::new(),
        }
    }

    fn startup(&self) {
        self.setup_accels();
        self.load_css();
        self.main.startup();
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
        self.main.shutdown();
        // TODO save state?
    }

    fn activate(&self) {
        let window = gtk::ApplicationWindow::new(&self.app);
        self.main.activate();

        // menu bar
        let menu = gio::Menu::new();
        let menu_bar = gio::Menu::new();

        // define quit action
        let quit = gio::SimpleAction::new("quit", None);
        let window_clone = window.clone();
        quit.connect_activate(move |_, _| {
            window_clone.destroy();
        });
        self.app.add_action(&quit);

        menu.append("Quit", "app.quit");
        self.app.set_app_menu(&menu);
        self.app.set_menubar(&menu_bar);

        self.main.add_window(&window);
    }

    fn setup_actions(&self) {
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
