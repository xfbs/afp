use crate::ui::*;
use crate::*;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SectionOverviewController {
    index: usize,
    view: SectionOverView,
    data: Rc<RefCell<DataStore>>,
}

impl SectionOverviewController {
    pub fn new(data: &Rc<RefCell<DataStore>>, index: usize) -> SectionOverviewController {
        SectionOverviewController {
            index: index,
            view: SectionOverView::new(),
            data: data.clone(),
        }
    }

    pub fn startup(&self) {
        self.view.setup();
    }

    pub fn activate(&self) {
        self.activate_title();
        self.activate_buttons();
    }

    pub fn shutdown(&self) {}

    pub fn view(&self) -> &SectionOverView {
        &self.view
    }

    fn activate_title(&self) {
        if let Some(section) = self.data.borrow().section(self.index) {
            self.view.set_title(section.name());
        }
    }

    fn activate_buttons(&self) {
        // every time we show the view, update the color for the buttons.
        let controller = self.clone();
        self.view.widget().connect_map(move |_| {
            let data = controller.data.borrow();
            if let Some(section) = data.section(controller.index) {
                // go through all the questions
                let mut i = 0;
                for (ss_id, ss) in section.subsections().iter().enumerate() {
                    // TODO use filter
                    let state = section.state(QuestionFilter::SubSection(ss_id));
                    controller.set_button_state(i, state);
                    i += 1;

                    for (sss_id, _) in ss.subsubsections().iter().enumerate() {
                        let state = section.state(QuestionFilter::SubSubSection(ss_id, sss_id));
                        controller.set_button_state(i, state);
                        i += 1;
                    }
                }
            }
        });
    }

    fn set_button_state(&self, index: usize, state: QuestionState) {
        self.view.button_remove_class(index, "green");
        self.view.button_remove_class(index, "yellow");
        self.view.button_remove_class(index, "green");
        self.view.button_add_class(
            index,
            match state {
                QuestionState::Green => "green",
                QuestionState::Yellow => "yellow",
                QuestionState::Red => "red",
            },
        );
    }

    /// Creates buttons for each question with specified target function.
    pub fn setup_buttons<F: Fn(QuestionFilter) + Clone + 'static>(&self, f: F) {
        // TODO: save fun?
        let data = self.data.borrow();
        if let Some(section) = data.section(self.index) {
            // go through all the questions
            for (ss_id, ss) in section.subsections().iter().enumerate() {
                let button = self.view.add_button(&format!("{}", ss_id + 1));
                button.set_tooltip_text(ss.name());
                let filter = QuestionFilter::SubSection(ss_id);
                let fun = f.clone();
                button.connect_clicked(move |_| {
                    fun(filter);
                });

                for (sss_id, sss) in ss.subsubsections().iter().enumerate() {
                    let button = self
                        .view
                        .add_button(&format!("{}.{}", ss_id + 1, sss_id + 1));
                    button.set_tooltip_text(sss.name());
                    let filter = QuestionFilter::SubSubSection(ss_id, sss_id);
                    let fun = f.clone();
                    button.connect_clicked(move |_| {
                        fun(filter);
                    });
                }
            }
        }
    }
}
