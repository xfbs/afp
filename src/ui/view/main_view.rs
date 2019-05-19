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
}

impl MainView {
    pub fn new() -> MainView {
        let area = gtk::Notebook::new();
        let overview = OverView::new();

        MainView {
            area: area,
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
