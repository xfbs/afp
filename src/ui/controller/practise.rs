use crate::ui::*;
use crate::*;
use std::cell::RefCell;
use std::rc::Rc;
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
        self.view.connect_choose(move |button, index| {
            if index != 0 {
                button.get_style_context().add_class("red");
            }
        });
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
