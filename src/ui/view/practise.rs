extern crate glib;
extern crate gtk;

use crate::ui::*;
use gtk::prelude::*;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

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
    back: gtk::Button,
    answer_fn: Rc<RefCell<Option<Box<dyn Fn(&gtk::Button, usize)>>>>,
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
            back: gtk::Button::new_from_icon_name("go-previous", gtk::IconSize::Button),
            answer_fn: Rc::new(RefCell::new(None)),
        }
    }

    pub fn setup(&self) {
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

    pub fn set_section(&self, text: &str) {
        self.section.set_text(text);
    }

    pub fn set_subsection(&self, text: &str) {
        self.subsection.set_text(text);
    }

    pub fn set_id(&self, text: &str) {
        self.id.set_text(text);
    }

    pub fn set_question(&self, text: &str) {
        self.question.set_text(text);
    }

    pub fn add_answer(&self, row: usize) {
        if self.answers.get_child_at(0, row as i32).is_none() {
            let button = gtk::Button::new();
            let me = self.clone();
            button.connect_clicked(move |button| {
                let answer_fn = me.answer_fn.borrow();
                if let Some(ref fun) = *answer_fn {
                    fun(button, row);
                }
            });
            self.answers.attach(&button, 0, row as i32, 1, 1);
            let label = gtk::Label::new(None);
            self.answers.attach(&label, 1, row as i32, 4, 1);
        }
    }

    pub fn set_answer(&self, row: usize, btn: &str, text: &str) {
        self.add_answer(row);

        if let Some(button) = self.get_answer_button(row) {
            button.show();
            button.get_style_context().remove_class("red");
            button.set_label(btn);
        }

        if let Some(label) = self.get_answer_label(row) {
            label.show();
            label.set_text(text);
        }
    }

    fn get_answer_button(&self, row: usize) -> Option<gtk::Button> {
        match self.answers.get_child_at(0, row as i32) {
            Some(widget) => widget.downcast().ok(),
            None => None,
        }
    }

    fn get_answer_label(&self, row: usize) -> Option<gtk::Label> {
        match self.answers.get_child_at(1, row as i32) {
            Some(widget) => widget.downcast().ok(),
            None => None,
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
    pub fn connect_choose<F: Fn(&gtk::Button, usize) + 'static>(&self, f: F) {
        *self.answer_fn.borrow_mut() = Some(Box::new(f));
    }
}

impl View for PractiseView {
    fn widget(&self) -> gtk::Widget {
        self.body.clone().upcast()
    }
}
