use crate::ui::*;
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SectionOverviewController {
    index: usize,
    view: SectionOverView,
    data: Rc<RefCell<DataStore>>,
}

impl SectionOverviewController {
    pub fn new(data: &Rc<RefCell<DataStore>>, index: usize) -> SectionOverviewController {
        SectionOverviewController {
            index: index,
            view: SectionOverView::new(),
            data: data.clone(),
        }
    }

    pub fn startup(&self) {
        self.view.setup();
    }

    pub fn activate(&self) {
        if let Some(section) = self.data.borrow().section(self.index) {
            self.view.set_title(section.name());
        }
    }

    pub fn shutdown(&self) {
    }

    pub fn view(&self) -> &SectionOverView {
        &self.view
    }
}
