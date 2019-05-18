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
        // every time we show the view
        let controller = self.clone();
        self.view.widget().connect_map(move |_| {
            let data = controller.data.borrow();
            if let Some(section) = data.section(controller.index) {
                // go through all the questions
                for (i, question) in section.questions().iter().enumerate() {
                    // if the button doesn't exist, create it.
                    if controller.view.get_button(i).is_none() {
                        let button = controller.view.add_button(question.id());
                        let i = i;
                        let index = controller.index;
                        button.connect_clicked(move |_| {
                            println!("button {} clicked! in {}", i, index);
                        });
                    }
                }
            }
        });
    }
}
