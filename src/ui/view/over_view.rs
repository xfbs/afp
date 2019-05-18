extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use std::f64::consts::PI;
use std::rc::Rc;
use std::cell::RefCell;
use crate::*;
use crate::ui::view::*;

#[derive(Clone)]
pub struct OverView {
    body: gtk::Grid,
    label: gtk::Label,
    title: gtk::Label,
    section_labels: Rc<RefCell<Vec<gtk::Label>>>,
    section_charts: Rc<RefCell<Vec<gtk::DrawingArea>>>,
}

impl OverView {
    pub fn new() -> OverView {
        let body = gtk::Grid::new();
        let label = gtk::Label::new("Übersicht");
        let title = gtk::Label::new(None);

        OverView {
            body: body,
            label: label,
            title: title,
            section_labels: Rc::new(RefCell::new(Vec::new())),
            section_charts: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn init(&self, datastore: &DataStore) {
        // this method may be called multiple times, so here we clean
        // out the trash
        self.body.foreach(|widget| {
            self.body.remove(widget);
        });

        self.section_labels.borrow_mut().clear();
        self.section_charts.borrow_mut().clear();

        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.body.set_column_homogeneous(true);
        self.title.set_text("Übersicht");
        self.title.get_style_context().add_class("title");
        self.title.set_hexpand(true);
        self.body.attach(&self.title, 0, 0, datastore.sections().len() as i32, 1);

        for (i, _section) in datastore.sections().iter().enumerate() {
            let label = gtk::Label::new(None);
            label.set_hexpand(true);
            self.body.attach(&label, i as i32, 1, 1, 1);
            self.section_labels.borrow_mut().push(label);

            let area = gtk::DrawingArea::new();
            area.set_size_request(100, 100);
            area.set_hexpand(true);
            self.body.attach(&area, i as i32, 2, 1, 1);
            self.section_charts.borrow_mut().push(area);
        }
    }

    pub fn update(&self, datastore: &DataStore) {
        for (i, section) in datastore.sections().iter().enumerate() {
            // title
            self.section_labels.borrow()[i].set_text(section.short());
            self.section_labels.borrow()[i].get_style_context().add_class("subtitle");

            let count = section.count();
            let count_green = section.count_by_state(QuestionState::Green);
            let count_yellow = section.count_by_state(QuestionState::Yellow);

            self.section_charts.borrow()[i].connect_draw(move |widget, cairo| {
                let width = 100 as f64;
                let height = 100 as f64;
                let lwidth = 6.0;

                // make sure we're centered
                cairo.translate(widget.get_allocated_width() as f64 / 2.0 - width / 2.0, 0.0);

                // rotate by centerpoint of circle to get angles right
                cairo.translate(width / 2.0, height / 2.0);
                cairo.rotate(1.5 * PI);
                cairo.translate(-width / 2.0, -height / 2.0);
                cairo.set_line_width(lwidth);
                let draw_part = |cairo: &cairo::Context, start: f64, stop: f64| {
                    cairo.arc(width / 2.0 + 0.5 * lwidth,
                              height / 2.0,
                              width / 2.0 - lwidth, 
                              start * 2.0 * PI, 
                              stop * 2.0 *  PI);
                };

                if count > 0 {
                    let end_green = (count_green as f64) / (count as f64);
                    draw_part(cairo, 0.0, end_green);
                    cairo.set_source_rgba(0.20, 0.80, 0.20, 0.8);
                    cairo.stroke();
                    let end_yellow = end_green + (count_yellow as f64) / (count as f64);
                    draw_part(cairo, end_green, end_yellow);
                    cairo.set_source_rgba(0.80, 0.89, 0.20, 0.8);
                    cairo.stroke();
                    draw_part(cairo, end_yellow, 1.0);
                    cairo.set_source_rgba(0.80, 0.20, 0.20, 0.8);
                    cairo.stroke();
                } else {
                    draw_part(cairo, 0.0, 1.0);
                    cairo.set_source_rgba(0.80, 0.20, 0.20, 0.8);
                    cairo.stroke();
                }

                Inhibit(false)
            });
        }
    }
}

impl View for OverView {
    fn widget(&self) -> gtk::Widget {
        self.body.clone().upcast()
    }
}

impl Labeled for OverView {
    fn label(&self) -> gtk::Label {
        self.label.clone()
    }
}
