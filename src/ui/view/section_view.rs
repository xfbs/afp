extern crate gtk;

use crate::*;
use crate::ui::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SectionView {
    index: usize,
    /// Label (for use in tab/notebook switcher)
    label: gtk::Label,
    /// For the different views available in the section.
    stack: gtk::Stack,
    /// Body of main view.
    body: gtk::Grid,
    /// Title of section (in main view).
    title: gtk::Label,
    /// Button to start exam mode.
    pub exam: gtk::Button,
    /// Button to start practise mode.
    practise: gtk::Button,
    /// Info view of questions and their current state.
    questions: gtk::FlowBox,

    question: QuestionView,
}

impl SectionView {
    pub fn new(index: usize) -> SectionView {
        SectionView {
            index: index,
            label: gtk::Label::new(None),
            stack: gtk::Stack::new(),
            body: gtk::Grid::new(),
            title: gtk::Label::new(None),
            exam: gtk::Button::new(),
            practise: gtk::Button::new(),
            questions: gtk::FlowBox::new(),
            question: QuestionView::new(),
        }
    }

    pub fn widget(&self) -> &gtk::Stack {
        &self.stack
    }

    pub fn label(&self) -> &gtk::Label {
        &self.label
    }

    pub fn init(&self, ds: &Rc<RefCell<DataStore>>) {
        // cleanup
        self.body.foreach(|widget| {
            self.body.remove(widget);
        });

        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.body.set_column_homogeneous(true);
        self.title.set_hexpand(true);
        self.body.attach(&self.title, 0, 0, 2, 1);
        self.questions.set_hexpand(true);
        self.body.attach(&self.questions, 0, 1, 2, 1);
        self.body.attach(&self.practise, 0, 2, 1, 1);
        self.body.attach(&self.exam, 1, 2, 1, 1);
        self.stack.add_named(&self.body, "main");

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
        /*
        let me = self.clone();
        let sec = section.clone();
        self.question.connect_next(move |_| {
            if let Some(question) = sec.practise() {
                me.show_question(question);
            } else {
                me.show_main();
            }
        });
        */
    }

    pub fn update(&self, ds: &Rc<RefCell<DataStore>>) {
        let datastore = ds.borrow();
        let section = datastore.section(self.index).unwrap();
        self.label.set_text(section.short());

        self.title.set_text(section.name());
        self.title.get_style_context().add_class("title");

        self.practise.set_label("Üben");
        self.exam.set_label("Prüfung Simulieren");

        // every time we reload this widget, we want to update the colors
        let me = self.clone();
        let index = self.index;
        let ds_clone = ds.clone();
        self.body.connect_map(move |_| {
            let ds = ds_clone.borrow();
            if let Some(section) = ds.section(index) {
                for (i, question) in section.questions().iter().enumerate() {
                    // if the button doesn't exist, create it.
                    if me.questions.get_child_at_index(i as i32).is_none() {
                        let button = gtk::Button::new();
                        button.set_label(question.id());
                        button.set_hexpand(false);
                        button.show();
                        me.questions.add(&button);
                        let me = me.clone();
                        let ds_clone = ds_clone.clone();
                        button.connect_clicked(move |_| {
                            let ds = ds_clone.borrow();
                            if let Some(section) = ds.section(me.index) {
                                if let Some(question) = section.question(i) {
                                    me.show_question(i, &question);
                                }
                            }
                        });
                    }

                    // set color of button.
                    if let Some(child) = me.questions.get_child_at_index(i as i32) {
                        if let Some(button) = child.get_child() {
                            let style = button.get_style_context();
                            style.remove_class("green");
                            style.remove_class("yellow");
                            style.remove_class("red");
                            style.add_class(match question.state() {
                                QuestionState::Green => "green",
                                QuestionState::Yellow => "yellow",
                                QuestionState::Red => "red"
                            });
                        }
                    }
                }
            }
        });

        // load a suggested question when the practise button
        // is pressed.
        let me = self.clone();
        let ds_clone = ds.clone();
        self.practise.connect_clicked(move |_| {
            let ds = ds_clone.borrow();
            if let Some(section) = ds.section(me.index) {
                let index = section.practise();
                if let Some(question) = section.question(index) {
                    me.show_question(index, question);
                }
            }
        });
    }

    fn show_question(&self, pos: usize, question: &Question) {
        self.question.update(pos, question);
        self.stack.set_visible_child_full("question", gtk::StackTransitionType::SlideLeft);
    }

    fn show_main(&self) {
        self.stack.set_visible_child_full("main", gtk::StackTransitionType::SlideRight);
    }
}

