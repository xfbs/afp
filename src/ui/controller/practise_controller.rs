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

    pub fn show(&self, index: usize) {
        let data = self.data.borrow();
        if let Some(section) = data.section(self.index) {
            if let Some(question) = section.question(index) {
                self.view.set_section(question.subsection());
                self.view.set_subsection(question.subsubsection());
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
}

impl Controller for PractiseController {
    fn startup(&self) {
        self.view.setup();
    }

    fn activate(&self) {
    }

    fn shutdown(&self) {
    }
}
