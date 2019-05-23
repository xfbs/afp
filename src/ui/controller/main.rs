use crate::ui::*;
use crate::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use ui::view::View;

#[derive(Clone)]
pub struct MainController {
    view: MainView,
    overview: OverviewController,
    sections: Rc<RefCell<Vec<SectionController>>>,
    data: Rc<RefCell<DataStore>>,
}

impl MainController {
    pub fn new() -> MainController {
        let data = Rc::new(RefCell::new(DataStore::new()));

        MainController {
            view: MainView::new(),
            overview: OverviewController::new(&data),
            sections: Rc::new(RefCell::new(Vec::new())),
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
        self.data
            .borrow()
            .save()
            .unwrap_or_else(|_| panic!("error saving file!"));
    }

    fn load_data(&self) {
        *self.data.borrow_mut() = DataStore::load(&std::path::PathBuf::from(
            "/Users/pelsen/.config/afp/datastore.yml",
        ))
        .unwrap();
    }

    fn activate_overview(&self) {
        self.overview.activate();
        self.view.add_tab(self.overview.view());
    }

    fn activate_sections(&self) {
        for (i, _) in self.data.borrow().sections().iter().enumerate() {
            self.activate_section(i);
        }
    }

    fn activate_section(&self, index: usize) {
        let section = SectionController::new(&self.data, index);
        section.startup();
        section.activate();
        self.view.add_tab(section.view());
        self.sections.borrow_mut().push(section);
    }

    pub fn add_window(&self, window: &gtk::ApplicationWindow) {
        window.add(&self.view.widget());
        window.set_default_size(500, 400);
        window.set_position(gtk::WindowPosition::Center);
        window.show_all();
    }
}
