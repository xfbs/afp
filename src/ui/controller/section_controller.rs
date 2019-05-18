use crate::ui::*;
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SectionController {
    view: SectionView,
    data: Rc<RefCell<DataStore>>,
}

impl SectionController {
    pub fn new(data: &Rc<RefCell<DataStore>>) -> SectionController {
        SectionController {
            view: SectionView::new(),
            data: data.clone(),
        }
    }

    pub fn startup(&self) {
    }

    pub fn activate(&self) {
    }

    pub fn shutdown(&self) {
    }

    pub fn view(&self) -> &SectionView {
        &self.view
    }
}
