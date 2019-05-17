extern crate gtk;

use gtk::prelude::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct QuestionView {
    body: gtk::Grid,
    section: gtk::Label,
    subsection: gtk::Label,
    id: gtk::Label,
    question: gtk::Label,
    button: gtk::Button,
    answer: gtk::Label,
    back: gtk::Button,
}

impl QuestionView {
    pub fn new() -> QuestionView {
        QuestionView {
            body: gtk::Grid::new(),
            section: gtk::Label::new(None),
            subsection: gtk::Label::new(None),
            id: gtk::Label::new(None),
            question: gtk::Label::new(None),
            button: gtk::Button::new(),
            answer: gtk::Label::new(None),
            back: gtk::Button::new(),
        }
    }

    pub fn init(&self) {
        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.body.attach(&self.section, 0, 0, 2, 1);
        self.body.attach(&self.subsection, 0, 1, 2, 1);
        self.body.attach(&self.id, 0, 2, 1, 1);
        self.body.attach(&self.question, 1, 2, 1, 1);
        self.body.attach(&self.button, 0, 3, 1, 1);
        self.body.attach(&self.answer, 1, 3, 1, 1);
        self.section.set_hexpand(true);
        self.section.get_style_context().add_class("subtitle");
        self.subsection.get_style_context().add_class("subtitle");
    }

    pub fn update(&self, question: &Question) {
        self.section.set_text(question.subsection());
        self.subsection.set_text(question.subsubsection());
        self.id.set_text(question.id());
        self.question.set_text(question.question());
        self.button.set_label("A");
        self.answer.set_text(&question.answers()[0]);
    }

    pub fn widget(&self) -> &gtk::Grid {
        &self.body
    }
}
