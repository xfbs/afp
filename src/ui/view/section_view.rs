extern crate gtk;

use crate::*;
use crate::ui::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SectionView {
    label: gtk::Label,
    stack: gtk::Stack,
}

impl SectionView {
    pub fn new() -> SectionView {
        SectionView {
            label: gtk::Label::new(None),
            stack: gtk::Stack::new(),
        }
    }

    pub fn widget(&self) -> &gtk::Stack {
        &self.stack
    }

    pub fn label(&self) -> &gtk::Label {
        &self.label
    }

    pub fn init(&self, ds: &Rc<RefCell<DataStore>>) {
        /*
        self.overview.init();
        self.stack.add_named(&self.overview.widget(), "main");

        self.question.init(ds);
        self.stack.add_named(self.question.widget(), "question");

        // connect the back button of the question.
        let me = self.clone();
        self.question.connect_back(move |_| {
            me.show_main();
        });

        let me = self.clone();
        let ds: Rc<RefCell<DataStore>> = ds.clone();
        self.question.connect_choose(move |question, choice| {
            println!("choose: {} {}", question, choice);

            let mut ds = ds.borrow_mut();
            match ds.section_mut(me.index) {
                Some(section) => {
                    match section.question_mut(question) {
                        Some(question) => {
                            question.answer(choice);
                        },
                        None => panic!(),
                    }

                    if choice == 0 {
                        let next = section.practise();
                        if let Some(question) = section.question(next) {
                            me.show_question(next, question);
                        } else {
                            me.show_main();
                        }
                    }
                },
                None => panic!(),
            }
        });
        */
    }

    pub fn set_label(&self, label: &str) {
        self.label.set_text(label);
    }

    pub fn add_named<T: View>(&self, page: &T, name: &str) {
        self.stack.add_named(&page.widget(), name);
    }

    pub fn show(&self, child: &str, transition: gtk::StackTransitionType) {
        self.stack.set_visible_child_full(child, transition);
    }
}

impl View for SectionView {
    fn widget(&self) -> gtk::Widget {
        self.stack.clone().upcast()
    }
}

impl Labeled for SectionView {
    fn label(&self) -> gtk::Label {
        self.label.clone()
    }
}
