use crate::ui::*;
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;

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

    pub fn shutdown(&self) {
    }

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
                    for (i, question) in section.questions().iter().enumerate() {
                        controller.view.button_remove_class(i, "green");
                        controller.view.button_remove_class(i, "yellow");
                        controller.view.button_remove_class(i, "green");
                        controller.view.button_add_class(i, match question.state() {
                            QuestionState::Green => "green",
                            QuestionState::Yellow => "yellow",
                            QuestionState::Red => "red"
                        });
                    }
                }
        });
    }

    /// Creates buttons for each question with specified target function.
    pub fn setup_buttons<F: Fn(usize) + Clone + 'static>(&self, fun: F) {
        // TODO: save fun?
        let data = self.data.borrow();
        if let Some(section) = data.section(self.index) {
            // go through all the questions
            for (i, question) in section.questions().iter().enumerate() {
                // if the button doesn't exist, create it.
                if self.view.get_button(i).is_none() {
                    let button = self.view.add_button(question.id());
                    let fun = fun.clone();
                    button.connect_clicked(move |_| {
                        fun(i);
                    });
                }
            }
        }
    }
}
