
use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::*;
use crate::*;

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
}
