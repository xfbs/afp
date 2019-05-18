use crate::ui::*;
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct OverviewController {
    view: OverView,
    data: Rc<RefCell<DataStore>>,
}

impl OverviewController {
    pub fn new(data: &Rc<RefCell<DataStore>>) -> OverviewController {
        OverviewController {
            view: OverView::new(),
            data: data.clone(),
        }
    }

    pub fn startup(&self) {
    }

    pub fn activate(&self) {
        self.view.init(&self.data.borrow());
        self.view.update(&self.data.borrow());
    }

    pub fn shutdown(&self) {
    }

    pub fn view(&self) -> &OverView {
        &self.view
    }
}
