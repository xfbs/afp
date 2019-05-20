extern crate gio;
extern crate gtk;

use crate::ui::view::*;
use gtk::prelude::*;

#[derive(Clone)]
pub struct MainView {
    pub area: gtk::Notebook,
}

impl MainView {
    pub fn new() -> MainView {
        MainView {
            area: gtk::Notebook::new(),
        }
    }

    pub fn add_tab<T: View + Labeled>(&self, page: &T) {
        self.area.append_page(&page.widget(), Some(&page.label()));
    }
}

impl View for MainView {
    fn widget(&self) -> gtk::Widget {
        self.area.clone().upcast()
    }
}
