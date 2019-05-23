use crate::ui::*;
use crate::*;
use gtk::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone)]
pub struct PractiseController {
    index: usize,
    view: PractiseView,
    data: Rc<RefCell<DataStore>>,
    filter: Rc<Cell<QuestionFilter>>,
}

impl PractiseController {
    pub fn new(data: &Rc<RefCell<DataStore>>, index: usize) -> PractiseController {
        PractiseController {
            index: index,
            view: PractiseView::new(),
            data: data.clone(),
            filter: Rc::new(Cell::new(QuestionFilter::All)),
        }
    }

    pub fn view(&self) -> &PractiseView {
        &self.view
    }

    pub fn show(&self) {
        let data = self.data.borrow();
        if let Some(section) = data.section(self.index) {
            // FIXME error handling?
            let index = section.practise(self.filter.get()).unwrap();
            if let Some(question) = section.question(index) {
                if let Some(subsection) = section.subsection(question.subsection()) {
                    self.view.set_section(subsection.name());

                    if let Some(subsubsection) = subsection.subsubsection(question.subsubsection())
                    {
                        self.view.set_subsection(subsubsection.name());
                    }
                }

                self.view.set_id(question.id());
                self.view.set_question(question.question());

                for (i, answer) in question.answers().iter().enumerate() {
                    self.view.set_answer(i, &format!("{}", i), answer);
                }
            }
        }
    }

    pub fn connect_back<F: Fn() + 'static>(&self, fun: F) {
        self.view.connect_back(move |_| {
            fun();
        });
    }

    pub fn activate_choose(&self) {
        let me = self.clone();
        self.view.connect_choose(move |button, index| {
            if index != 0 {
                // answer is wrong. mark button.
                button.get_style_context().add_class("red");
            } else {
                me.show();
            }
        });
    }

    pub fn set_filter(&self, filter: QuestionFilter) {
        self.filter.set(filter);
    }
}

impl Controller for PractiseController {
    fn startup(&self) {
        self.view.setup();
    }

    fn activate(&self) {
        self.activate_choose();
    }

    fn shutdown(&self) {}
}
