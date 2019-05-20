use crate::ui::*;
use crate::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SectionController {
    index: usize,
    view: SectionView,
    overview: SectionOverviewController,
    practise: PractiseController,
    data: Rc<RefCell<DataStore>>,
    filter: Rc<Cell<QuestionFilter>>,
}

impl SectionController {
    pub fn new(data: &Rc<RefCell<DataStore>>, index: usize) -> SectionController {
        SectionController {
            index: index,
            view: SectionView::new(),
            overview: SectionOverviewController::new(data, index),
            practise: PractiseController::new(data, index),
            data: data.clone(),
            filter: Rc::new(Cell::new(QuestionFilter::All)),
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

    /// Switch to the section overview.
    pub fn show_overview(&self) {
        self.view.show("main", gtk::StackTransitionType::SlideRight);
    }

    /// Switch to the practise view.
    pub fn show_practise(&self, num: usize) {
        self.practise.show(num);
        self.view
            .show("practise", gtk::StackTransitionType::SlideLeft);
    }

    /// Switch to the practise view with a (suggested) question.
    pub fn show_next_practise(&self) {
        let data = self.data.borrow();
        let filter = self.filter.get();
        if let Some(section) = data.section(self.index) {
            let index = section.practise(filter);
            self.show_practise(index);
        }
    }

    fn set_filter(&self, filter: QuestionFilter) {
        self.filter.set(filter);
    }

    fn activate_overview_buttons(&self) {
        let controller = self.clone();
        self.overview.setup_buttons(move |filter| {
            controller.set_filter(filter);
            controller.show_next_practise();
        });

        let controller = self.clone();
        self.overview.view().connect_practise(move || {
            controller.set_filter(QuestionFilter::All);
            controller.show_next_practise();
        });
    }

    fn activate_views(&self) {
        self.view.add_named(self.overview.view(), "main");
        self.view.add_named(self.practise.view(), "practise");
    }

    fn activate_practise_buttons(&self) {
        let controller = self.clone();
        self.practise.connect_back(move || {
            controller.show_overview();
        });
    }
}

impl Controller for SectionController {
    fn startup(&self) {
        self.overview.startup();
        self.practise.startup();
    }

    fn activate(&self) {
        self.overview.activate();
        self.activate_overview_buttons();

        self.practise.activate();
        self.activate_practise_buttons();

        self.activate_label();
        self.activate_views();
    }

    fn shutdown(&self) {
        self.overview.shutdown();
        self.practise.shutdown();
    }
}
