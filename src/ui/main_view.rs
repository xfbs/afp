extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::*;
use crate::ui::*;

#[derive(Debug, Clone)]
pub struct MainView {
    pub area: gtk::Notebook,
    pub overview: OverView,
    pub sections: Rc<RefCell<Vec<SectionView>>>
}

impl MainView {
    pub fn new() -> MainView {
        let area = gtk::Notebook::new();
        let overview = OverView::new();

        MainView {
            area: area,
            overview: overview,
            sections: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn add_section(&self, sec: &Section) {
        let section = SectionView::new();
        section.init(sec);
        section.update(sec);
        self.area.append_page(section.widget(), Some(section.label()));
        self.sections.borrow_mut().push(section);
    }

    pub fn init(&self, datastore: Rc<RefCell<DataStore>>) {
        self.overview.init(&datastore.borrow());
        self.overview.update(&datastore.borrow());
        self.area.append_page(self.overview.widget(), Some(self.overview.label()));

        for section in datastore.borrow().sections() {
            self.add_section(section);
        }
    }
}

