use crate::ui::*;
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SectionController {
    index: usize,
    view: SectionView,
    overview: SectionOverviewController,
    practise: PractiseController,
    data: Rc<RefCell<DataStore>>,
}

impl SectionController {
    pub fn new(data: &Rc<RefCell<DataStore>>, index: usize) -> SectionController {
        SectionController {
            index: index,
            view: SectionView::new(),
            overview: SectionOverviewController::new(data, index),
            practise: PractiseController::new(data, index),
            data: data.clone(),
        }
    }

    pub fn view(&self) -> &SectionView {
        &self.view
    }

    fn activate_label(&self) {
        if let Some(section) = self.data.borrow().section(self.index) {
            self.view.set_label(section.short());
        }
    }
}

impl Controller for SectionController {
    fn startup(&self) {
        self.overview.startup();
        self.practise.startup();
    }

    fn activate(&self) {
        self.overview.activate();
        self.practise.activate();
        self.activate_label();
        self.view.add_named(self.overview.view(), "main");
        self.view.add_named(self.practise.view(), "practise");
    }

    fn shutdown(&self) {
        self.overview.shutdown();
        self.practise.shutdown();
    }
}
