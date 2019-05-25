extern crate permutation;
extern crate rand;

use crate::ui::*;
use crate::*;
use gtk::prelude::*;
use permutation::Permutation;
use rand::seq::SliceRandom;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Clone)]
pub struct PractiseController {
    section: usize,
    question: Rc<Cell<usize>>,
    permutation: Rc<RefCell<Permutation>>,
    view: PractiseView,
    data: Rc<RefCell<DataStore>>,
    filter: Rc<Cell<QuestionFilter>>,
}

impl PractiseController {
    pub fn new(data: &Rc<RefCell<DataStore>>, section: usize) -> PractiseController {
        PractiseController {
            section: section,
            question: Rc::new(Cell::new(0)),
            permutation: Rc::new(RefCell::new(Permutation::one(0))),
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
        if let Some(section) = data.section(self.section) {
            // get new question
            section
                .practise(self.filter.get())
                .map(|question| self.question.set(question))
                .unwrap_or_else(|| panic!("can't load question!"));

            // display question
            if let Some(question) = section.question(self.question.get()) {
                if let Some(subsection) = section.subsection(question.subsection()) {
                    self.view.set_section(subsection.name());

                    if let Some(subsubsection) = subsection.subsubsection(question.subsubsection())
                    {
                        self.view.set_subsection(subsubsection.name());
                    }
                }

                self.view.set_id(question.id());
                self.view.set_question(question.question());
                self.create_permutation(question.answers.len());
                let permutation = self.permutation.borrow();

                for (num, answer) in question.answers().iter().enumerate() {
                    let index = permutation.apply_idx(num);
                    let alph = (0..25)
                        .map(|offset| (('A' as u8) + offset) as char)
                        .nth(index)
                        .unwrap_or('?');
                    self.view.set_answer(index, &format!("{}", alph), answer);
                }
            }
        }
    }

    fn create_permutation(&self, size: usize) {
        let mut rng = rand::thread_rng();
        let mut permuted = (0..size).collect::<Vec<usize>>();
        permuted.shuffle(&mut rng);
        let permutation = Permutation::from_vec(permuted);
        self.permutation.replace(permutation);
    }

    pub fn connect_back<F: Fn() + 'static>(&self, fun: F) {
        self.view.connect_back(move |_| {
            fun();
        });
    }

    pub fn activate_choose(&self) {
        let me = self.clone();
        self.view.connect_choose(move |button, index| {
            // compute actual answer number
            let num = me.permutation.borrow().apply_inv_idx(index);

            {
                // record answer (in a different scope so the borrowed mut
                // data doesn't prevent it from being able to borrow it as
                // immutable later).
                let mut data = me.data.borrow_mut();
                if let Some(section) = data.section_mut(me.section) {
                    if let Some(question) = section.question_mut(me.question.get()) {
                        question.answer(num);
                    }
                }
            }

            // mark button as red or show next question if it was correct.
            if num != 0 {
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
