extern crate gtk;
extern crate glib;

use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use crate::*;
use crate::ui::*;

#[derive(Clone)]
pub struct PractiseView {
    body: gtk::Grid,
    title_box: gtk::Box,
    title: gtk::Label,
    section: gtk::Label,
    subsection: gtk::Label,
    id: gtk::Label,
    question: gtk::Label,
    answers: gtk::Grid,
    choose: Rc<RefCell<Vec<gtk::Button>>>,
    answer: Rc<RefCell<Vec<gtk::Label>>>,
    choose_fn: Rc<RefCell<Option<Box<dyn Fn(usize, usize) + 'static>>>>,
    back: gtk::Button,
}

impl PractiseView {
    pub fn new() -> PractiseView {
        PractiseView {
            body: gtk::Grid::new(),
            title_box: gtk::Box::new(gtk::Orientation::Horizontal, 10),
            title: gtk::Label::new(None),
            section: gtk::Label::new(None),
            subsection: gtk::Label::new(None),
            id: gtk::Label::new(None),
            answers: gtk::Grid::new(),
            question: gtk::Label::new(None),
            choose: Rc::new(RefCell::new(Vec::new())),
            answer: Rc::new(RefCell::new(Vec::new())),
            choose_fn: Rc::new(RefCell::new(None)),
            back: gtk::Button::new_from_icon_name("go-previous", gtk::IconSize::Button),
        }
    }

    pub fn init(&self, _ds: &Rc<RefCell<DataStore>>) {
        self.title.set_text("Übung");
        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.answers.set_column_homogeneous(true);
        self.answers.set_column_spacing(10);
        self.answers.set_row_spacing(5);
        self.title_box.add(&self.back);
        self.title_box.set_center_widget(&self.title);
        self.body.attach(&self.title_box, 0, 0, 4, 1);
        self.body.attach(&self.section, 1, 1, 2, 1);
        self.body.attach(&self.subsection, 1, 2, 2, 1);
        self.body.attach(&self.id, 1, 3, 1, 1);
        self.body.attach(&self.question, 2, 3, 1, 1);
        self.body.attach(&self.answers, 1, 4, 2, 1);
        self.section.set_hexpand(true);
        self.title.get_style_context().add_class("title");
        self.section.get_style_context().add_class("subtitle");
        self.subsection.get_style_context().add_class("subtitle");
    }

    pub fn update(&self, pos: usize, question: &Question) {
        self.section.set_text(question.subsection());
        self.subsection.set_text(question.subsubsection());
        self.id.set_text(question.id());
        self.question.set_text(question.question());

        // setup answer button
        for (i, answer) in question.answers().iter().enumerate() {
            if self.choose.borrow().get(i).is_none() {
                let button = gtk::Button::new();
                self.answers.attach(&button, 0, i as i32, 1, 1);
                let f = self.choose_fn.clone();
                button.connect_clicked(move |_| {
                    if let Some(ref f) = *f.borrow() {
                        f(pos, i);
                    }
                });
                self.choose.borrow_mut().push(button);
            }

            if let Some(button) = self.choose.borrow().get(i) {
                button.set_label(&format!("{}", i + 1));
                button.show();
            }

            if self.answer.borrow().get(i).is_none() {
                let label = gtk::Label::new(None);
                self.answers.attach(&label, 1, i as i32, 4, 1);
                label.set_hexpand(false);
                self.answer.borrow_mut().push(label);
            }

            if let Some(label) = self.answer.borrow().get(i) {
                label.set_text(&answer);
                label.show();
            }
        }
    }

    pub fn widget(&self) -> &gtk::Grid {
        &self.body
    }

    /// Connect a closure to the back button.
    pub fn connect_back<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.back.connect_clicked(f);
    }

    /// Connect a closure to when a choice is made. The argument is the numeric
    /// index of the choice, with 0 being the first (and correct) one always.
    pub fn connect_choose<F: Fn(usize, usize) + 'static>(&self, f: F) {
        *self.choose_fn.borrow_mut() = Some(Box::new(f));
    }
}

impl View for PractiseView {
    fn widget(&self) -> gtk::Widget {
        self.body.clone().upcast()
    }
}
