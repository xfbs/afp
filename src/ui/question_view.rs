extern crate gtk;

use gtk::prelude::*;

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
}

