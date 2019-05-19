use crate::ui::*;
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;

#[derive(Clone)]
pub struct PractiseController {
    index: usize,
    view: PractiseView,
    data: Rc<RefCell<DataStore>>,
}

impl PractiseController {
    pub fn new(data: &Rc<RefCell<DataStore>>, index: usize) -> PractiseController {
        PractiseController {
            index: index,
            view: PractiseView::new(),
            data: data.clone(),
        }
    }

    pub fn view(&self) -> &PractiseView {
        &self.view
    }
}

impl Controller for PractiseController {
    fn startup(&self) {
    }

    fn activate(&self) {
    }

    fn shutdown(&self) {
    }
}
