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
    subsections: gtk::FlowBox,
    exam: gtk::Button,
    practise: gtk::Button,
}

impl SectionOverView {
    pub fn new() -> SectionOverView {
        SectionOverView {
            body: gtk::Grid::new(),
            title: gtk::Label::new(None),
            subsections: gtk::FlowBox::new(),
            exam: gtk::Button::new(),
            practise: gtk::Button::new(),
        }
    }

    pub fn setup(&self) {
        self.title.set_hexpand(true);
        self.title.get_style_context().add_class("title");
        self.subsections.set_hexpand(true);
        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.body.set_column_homogeneous(true);
        self.body.attach(&self.title, 0, 0, 2, 1);
        self.body.attach(&self.subsections, 0, 1, 2, 1);
        self.body.attach(&self.practise, 0, 2, 1, 1);
        self.body.attach(&self.exam, 1, 2, 1, 1);
        self.exam.set_label("Prüfung");
        self.practise.set_label("Üben");
    }

    pub fn set_title(&self, title: &str) {
        self.title.set_text(title);
    }

    pub fn button_add_class(&self, index: usize, class: &str) {
        if let Some(button) = self.get_button(index) {
            let style = button.get_style_context();
            style.add_class(class);
        } else {
            panic!();
        }
    }

    pub fn button_remove_class(&self, index: usize, class: &str) {
        if let Some(button) = self.get_button(index) {
            let style = button.get_style_context();
            style.remove_class(class);
        } else {
            panic!();
        }
    }

    pub fn get_button(&self, index: usize) -> Option<gtk::Widget> {
        match self.subsections.get_child_at_index(index as i32) {
            Some(child) => child.get_child(),
            None => None,
        }
    }

    pub fn add_button(&self, label: &str) -> gtk::Button {
        let button = gtk::Button::new();
        button.set_label(label);
        button.set_hexpand(false);
        button.show();
        self.subsections.add(&button);
        button
    }

    pub fn connect_exam<F: Fn() + 'static>(&self, fun: F) {
        self.exam.connect_clicked(move |_| {
            fun();
        });
    }

    pub fn connect_practise<F: Fn() + 'static>(&self, fun: F) {
        self.practise.connect_clicked(move |_| {
            fun();
        });
    }
}

impl View for SectionOverView {
    fn widget(&self) -> gtk::Widget {
        self.body.clone().upcast()
    }
}
