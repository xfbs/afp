extern crate gtk;

use crate::*;
use crate::ui::*;
use gtk::prelude::*;

#[derive(Debug, Clone)]
pub struct SectionView {
    /// Label (for use in tab/notebook switcher)
    label: gtk::Label,
    /// For the different views available in the section.
    stack: gtk::Stack,
    /// Body of main view.
    body: gtk::Grid,
    /// Title of section (in main view).
    title: gtk::Label,
    /// Button to start exam mode.
    pub exam: gtk::Button,
    /// Button to start practise mode.
    practise: gtk::Button,
    /// Info view of questions and their current state.
    questions: gtk::FlowBox,

    question: QuestionView,
}

impl SectionView {
    pub fn new() -> SectionView {
        SectionView {
            /// Label (for use in tab/notebook switcher)
            label: gtk::Label::new(None),

            /// For the different views available in the section.
            stack: gtk::Stack::new(),

            /// Body of main view.
            body: gtk::Grid::new(),

            /// Title of section (in main view).
            title: gtk::Label::new(None),

            /// Button to start exam mode.
            exam: gtk::Button::new(),

            /// Button to start practise mode.
            practise: gtk::Button::new(),

            /// Info view of questions and their current state.
            questions: gtk::FlowBox::new(),

            question: QuestionView::new(),
        }
    }

    pub fn widget(&self) -> &gtk::Stack {
        &self.stack
    }

    pub fn label(&self) -> &gtk::Label {
        &self.label
    }

    pub fn init(&self, _section: &Section) {
        // cleanup
        self.body.foreach(|widget| {
            self.body.remove(widget);
        });

        self.body.set_margin_top(10);
        self.body.set_margin_bottom(10);
        self.body.set_margin_start(10);
        self.body.set_margin_end(10);
        self.body.set_column_spacing(20);
        self.body.set_row_spacing(20);
        self.body.set_column_homogeneous(true);
        self.title.set_hexpand(true);
        self.body.attach(&self.title, 0, 0, 2, 1);
        self.questions.set_hexpand(true);
        self.body.attach(&self.questions, 0, 1, 2, 1);
        self.body.attach(&self.practise, 0, 2, 1, 1);
        self.body.attach(&self.exam, 1, 2, 1, 1);
        self.stack.add_named(&self.body, "main");

        self.question.init();
        self.stack.add_named(self.question.widget(), "question");

        // connect the back button of the question.
        let me = self.clone();
        self.question.connect_back(move |btn| {
            me.show_main();
        });
    }

    pub fn update(&self, section: &Section) {
        self.label.set_text(section.short());

        self.title.set_text(section.name());
        self.title.get_style_context().add_class("title");

        self.practise.set_label("Üben");
        self.exam.set_label("Prüfung Simulieren");

        for question in section.questions() {
            let button = gtk::Button::new();
            button.set_label(question.id());
            button.set_hexpand(false);

            let class = match question.state() {
                QuestionState::Green => "green",
                QuestionState::Yellow => "yellow",
                QuestionState::Red => "red"
            };
            button.get_style_context().add_class(class);

            let me: SectionView = self.clone();
            let question: Question = question.clone();
            button.connect_clicked(move |_| {
                me.show_question(&question);
            });
            self.questions.add(&button);
        }

        //let me = self.clone();
        //let sec = section.clone();
        //self.question.connect_back(move |btn| {
        //    let len = section.questions().len();
        //});
    }

    fn show_question(&self, question: &Question) {
        self.question.update(question);
        self.stack.set_visible_child_full("question", gtk::StackTransitionType::SlideLeft);
    }

    fn show_main(&self) {
        self.stack.set_visible_child_full("main", gtk::StackTransitionType::SlideRight);
    }
}

