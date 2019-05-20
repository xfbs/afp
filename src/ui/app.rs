extern crate gio;
extern crate gtk;

use crate::ui::*;
use crate::*;
use gio::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

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
            app: gtk::Application::new(name, gio::ApplicationFlags::FLAGS_NONE)
                .expect("application startup failed"),
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
        // create window and store handle
        let window = gtk::ApplicationWindow::new(&self.app);
        *self.window.borrow_mut() = Some(window.clone());
        self.main.activate();
        self.setup_menu(&window);
        self.setup_actions();
        self.main.add_window(&window);
    }

    fn setup_menu(&self, window: &gtk::ApplicationWindow) {
        let menu = gio::Menu::new();
        let menu_bar = gio::Menu::new();
        menu.append("About", "app.about");
        menu.append("Quit", "app.quit");
        self.app.set_app_menu(&menu);
        self.app.set_menubar(&menu_bar);
    }

    fn setup_actions(&self) {
        let quit = gio::SimpleAction::new("quit", None);
        let app = self.clone();
        quit.connect_activate(move |_, _| {
            if let Some(window) = app.window.borrow().clone() {
                window.destroy();
            }
        });

        let about = gio::SimpleAction::new("about", None);
        let app = self.clone();
        about.connect_activate(move |_, _| {
            let dialog = gtk::AboutDialog::new();
            dialog.set_authors(&[env!("CARGO_PKG_AUTHORS")]);
            dialog.set_website_label(Some("Webseite"));
            dialog.set_website(Some(env!("CARGO_PKG_REPOSITORY")));
            dialog.set_license_type(gtk::License::MitX11);
            dialog.set_program_name("Amateurfunkprüfer");
            dialog.set_version(env!("CARGO_PKG_VERSION"));
            dialog.set_comments(env!("CARGO_PKG_DESCRIPTION"));
            dialog.set_title("Über Amateurfunkprüfer");
            if let Some(window) = app.window.borrow().as_ref() {
                dialog.set_transient_for(Some(window));
            }
            dialog.run();
            dialog.destroy();
        });
        self.app.add_action(&quit);
        self.app.add_action(&about);
    }

    pub fn init(&self) {
        let app = self.clone();
        self.app.connect_startup(move |_| app.startup());
        let app = self.clone();
        self.app.connect_shutdown(move |_| {
            app.shutdown();
        });
        let app = self.clone();
        self.app.connect_activate(move |_| app.activate());
    }

    pub fn run(&self) {
        self.app.run(&env::args().collect::<Vec<_>>());
    }
}
