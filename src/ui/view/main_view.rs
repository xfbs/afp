extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::*;
use crate::ui::*;
use crate::ui::view::*;

#[derive(Clone)]
pub struct MainView {
    pub area: gtk::Notebook,
    pub overview: OverView,
    pub sections: Vec<SectionView>
}

impl MainView {
    pub fn new() -> MainView {
        let area = gtk::Notebook::new();
        let overview = OverView::new();

        MainView {
            area: area,
            overview: overview,
            sections: Vec::new(),
        }
    }

    pub fn add_tab<T: View + Labeled>(&self, page: &T) {
        self.area.append_page(&page.widget(), Some(&page.label()));
    }

    fn add_section(&mut self, ds: &Rc<RefCell<DataStore>>, i: usize) {
        let section = SectionView::new();
        section.init(ds);
        section.update(ds);
        self.area.append_page(section.widget(), Some(section.label()));
        self.sections.push(section);
    }

    pub fn init(&mut self, datastore: Rc<RefCell<DataStore>>) {
        self.overview.init(&datastore.borrow());
        self.overview.update(&datastore.borrow());
        self.area.append_page(&self.overview.widget(), Some(&self.overview.label()));

        for (i, _) in datastore.borrow().sections().iter().enumerate() {
            self.add_section(&datastore, i);
        }
    }
}

impl View for MainView {
    fn widget(&self) -> gtk::Widget {
        self.area.clone().upcast()
    }
}
