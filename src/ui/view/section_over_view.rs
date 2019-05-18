extern crate gtk;

use crate::*;
use crate::ui::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SectionOverView {
    body: gtk::Grid,
    title: gtk::Label,
    questions: gtk::FlowBox,
    exam: gtk::Button,
    practise: gtk::Button,
}

impl SectionOverView {
    pub fn new() -> SectionOverView {
        SectionOverView {
            body: gtk::Grid::new(),
            title: gtk::Label::new(None),
            questions: gtk::FlowBox::new(),
            exam: gtk::Button::new(),
            practise: gtk::Button::new(),
        }
    }

    pub fn setup(&self) {
        self.title.set_hexpand(true);
        self.title.get_style_context().add_class("title");
        self.questions.set_hexpand(true);
        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.body.set_column_homogeneous(true);
        self.body.attach(&self.title, 0, 0, 2, 1);
        self.body.attach(&self.questions, 0, 1, 2, 1);
        self.body.attach(&self.practise, 0, 2, 1, 1);
        self.body.attach(&self.exam, 1, 2, 1, 1);
        self.exam.set_label("Prüfung");
        self.practise.set_label("Üben");
    }

    pub fn set_title(&self, title: &str) {
        self.title.set_text(title);
    }
}

impl View for SectionOverView {
    fn widget(&self) -> gtk::Widget {
        self.body.clone().upcast()
    }
}
