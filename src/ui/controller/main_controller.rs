use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gio::prelude::*;
use crate::ui::*;
use crate::*;
use ui::view::View;

#[derive(Clone)]
pub struct MainController {
    view: MainView,
    overview: OverviewController,
    data: Rc<RefCell<DataStore>>
}

impl MainController {
    pub fn new() -> MainController {
        let data = Rc::new(RefCell::new(DataStore::new()));

        MainController {
            view: MainView::new(),
            overview: OverviewController::new(&data),
            data: data,
        }
    }

    pub fn view(&self) -> &MainView {
        &self.view
    }

    pub fn startup(&self) {
        self.overview.startup();
    }

    pub fn activate(&self) {
        self.load_data();
        self.activate_overview();
        self.activate_sections();
    }

    pub fn shutdown(&self) {
        self.overview.shutdown();
    }

    fn load_data(&self) {
        *self.data.borrow_mut() = DataStore::load(&std::path::PathBuf::from("/Users/pelsen/.config/afp/datastore.yml")).unwrap();
    }

    fn activate_overview(&self) {
        self.overview.activate();
        self.view.add_tab(self.overview.view());
    }

    fn activate_sections(&self) {
    }

    pub fn add_window(&self, window: &gtk::ApplicationWindow) {
        window.add(&self.view.widget());
        window.set_default_size(500, 400);
        window.set_position(gtk::WindowPosition::Center);
        window.show_all();
    }
}
