extern crate gtk;

use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct QuestionView {
    body: gtk::Grid,
    title_box: gtk::Box,
    title: gtk::Label,
    section: gtk::Label,
    subsection: gtk::Label,
    id: gtk::Label,
    question: gtk::Label,
    choose: Rc<RefCell<Vec<gtk::Button>>>,
    answer: gtk::Label,
    back: gtk::Button,
}

impl QuestionView {
    pub fn new() -> QuestionView {
        QuestionView {
            body: gtk::Grid::new(),
            title_box: gtk::Box::new(gtk::Orientation::Horizontal, 10),
            title: gtk::Label::new(None),
            section: gtk::Label::new(None),
            subsection: gtk::Label::new(None),
            id: gtk::Label::new(None),
            question: gtk::Label::new(None),
            choose: Rc::new(RefCell::new(Vec::new())),
            answer: gtk::Label::new(None),
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
        self.title_box.add(&self.back);
        self.title_box.set_center_widget(&self.title);
        self.body.attach(&self.title_box, 0, 0, 4, 1);
        self.body.attach(&self.section, 1, 1, 2, 1);
        self.body.attach(&self.subsection, 1, 2, 2, 1);
        self.body.attach(&self.id, 1, 3, 1, 1);
        self.body.attach(&self.question, 2, 3, 1, 1);
        self.body.attach(&self.answer, 2, 4, 1, 1);
        self.section.set_hexpand(true);
        self.title.get_style_context().add_class("title");
        self.section.get_style_context().add_class("subtitle");
        self.subsection.get_style_context().add_class("subtitle");
    }

    pub fn update(&self, question: &Question) {
        self.section.set_text(question.subsection());
        self.subsection.set_text(question.subsubsection());
        self.id.set_text(question.id());
        self.question.set_text(question.question());
        self.answer.set_text(&question.answers()[0]);

        // setup answer button
        for (i, answer) in question.answers().iter().enumerate() {
            if self.choose.borrow().get(i).is_none() {
                let button = gtk::Button::new();
                self.body.attach(&button, 1, 4 + i as i32, 1, 1);
                self.choose.borrow_mut().push(button);
            }

            if let Some(button) = self.choose.borrow().get(i) {
                button.set_label(&format!("{}", i + 1));
                button.show();
            }
        }
    }

    pub fn widget(&self) -> &gtk::Grid {
        &self.body
    }

    pub fn connect_back<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.back.connect_clicked(f);
    }

    pub fn connect_next<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        if let Some(button) = self.choose.borrow().get(0) {
            button.connect_clicked(f);
        }
    }

    pub fn connect_choose<F: Fn(usize) + Copy + 'static>(&self, f: F) {
        for (i, button) in self.choose.borrow().iter().enumerate() {
            button.connect_clicked(move |btn| {
                f(i);
            });
        }
    }
}
