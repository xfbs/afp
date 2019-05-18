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
    data: Rc<RefCell<DataStore>>
}

impl MainController {
    pub fn new() -> MainController {
        MainController {
            view: MainView::new(),
            data: Rc::new(RefCell::new(DataStore::new()))
        }
    }

    pub fn view(&self) -> &MainView {
        &self.view
    }

    pub fn startup(&self) {
    }

    pub fn activate(&self) {
        self.load_data();
        self.view.clone().init(self.data.clone());
    }

    pub fn shutdown(&self) {
    }

    fn load_data(&self) {
        *self.data.borrow_mut() = DataStore::load(&std::path::PathBuf::from("/Users/pelsen/.config/afp/datastore.yml")).unwrap();
    }

    pub fn add_window(&self, window: &gtk::ApplicationWindow) {
        window.add(&self.view.widget());
        window.set_default_size(500, 400);
        window.set_position(gtk::WindowPosition::Center);
        window.show_all();
    }
}
