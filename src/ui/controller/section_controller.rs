use crate::ui::*;
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SectionController {
    index: usize,
    view: SectionView,
    overview: SectionOverviewController,
    data: Rc<RefCell<DataStore>>,
}

impl SectionController {
    pub fn new(data: &Rc<RefCell<DataStore>>, index: usize) -> SectionController {
        SectionController {
            index: index,
            view: SectionView::new(),
            overview: SectionOverviewController::new(data, index),
            data: data.clone(),
        }
    }

    pub fn startup(&self) {
        self.overview.startup();
    }

    pub fn activate(&self) {
        self.overview.activate();
        if let Some(section) = self.data.borrow().section(self.index) {
            self.view.set_label(section.short());
        }

        self.view.add_named(self.overview.view(), "main");
    }

    pub fn shutdown(&self) {
        self.overview.shutdown();
    }

    pub fn view(&self) -> &SectionView {
        &self.view
    }
}
