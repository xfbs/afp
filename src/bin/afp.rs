extern crate afp;
extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use afp::*;
use std::env;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Mutex;
use std::cell::RefCell;

/// CSS style for this app.
const STYLE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/style.css"));

#[derive(Debug, Clone)]
struct App {
    app: gtk::Application,
    main: MainView
}

#[derive(Debug, Clone)]
struct MainView {
    area: gtk::Notebook,
    overview: OverView,
    sections: Rc<RefCell<Vec<SectionView>>>
}

#[derive(Debug, Clone)]
struct OverView {
    body: gtk::Grid,
    label: gtk::Label,
    title: gtk::Label,
    section_labels: Rc<RefCell<Vec<gtk::Label>>>,
    section_charts: Rc<RefCell<Vec<gtk::DrawingArea>>>,
}

#[derive(Debug, Clone)]
struct SectionView {
    /// Label (for use in tab/notebook switcher)
    label: gtk::Label,
    /// For the different views available in the section.
    stack: gtk::Stack,
    /// Body of main view.
    body: gtk::Grid,
    /// Title of section (in main view).
    title: gtk::Label,
    /// Button to start exam mode.
    exam: gtk::Button,
    /// Button to start practise mode.
    practise: gtk::Button,
    /// Info view of questions and their current state.
    questions: gtk::FlowBox,

    question: QuestionView,
}

#[derive(Debug, Clone)]
struct QuestionView {
    body: gtk::Grid,
    section: gtk::Label,
    subsection: gtk::Label,
    id: gtk::Label,
    question: gtk::Label,
    button: gtk::Button,
    answer: gtk::Label,
    back: gtk::Button,
}

impl OverView {
    fn new() -> OverView {
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

    fn init(&self, datastore: &DataStore) {
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

    fn update(&self, datastore: &DataStore) {
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

impl QuestionView {
    fn new() -> QuestionView {
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

impl SectionView {
    fn new() -> SectionView {
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

    fn widget(&self) -> &gtk::Stack {
        &self.stack
    }

    fn label(&self) -> &gtk::Label {
        &self.label
    }

    fn init(&self, _section: &Section) {
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
    }

    fn update(&self, section: &Section) {
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
    }

    fn show_question(&self, question: &Question) {
    }
}

impl MainView {
    fn new() -> MainView {
        let area = gtk::Notebook::new();
        let overview = OverView::new();

        MainView {
            area: area,
            overview: overview,
            sections: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn add_section(&self, sec: &Section) {
        let section = SectionView::new();
        section.init(sec);
        section.update(sec);
        self.area.append_page(section.widget(), Some(section.label()));
        self.sections.borrow_mut().push(section);
    }

    fn init(&self, datastore: &DataStore) {
        self.overview.init(datastore);
        self.overview.update(datastore);
        self.area.append_page(&self.overview.body, Some(&self.overview.label));

        for section in datastore.sections() {
            self.add_section(section);
        }
    }
}

impl App {
    fn new(name: &str) -> App {
        App {
            app: gtk::Application::new(name, gio::ApplicationFlags::FLAGS_NONE).expect("application startup failed"),
            main: MainView::new()
        }
    }

    fn startup(app: &gtk::Application) {
        app.set_accels_for_action("app.quit", &["<Primary>Q"]);

        // load css
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn shutdown(_app: &gtk::Application) {
        // TODO save state?
    }

    fn activate(app: &gtk::Application, mainview: &MainView) {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Amateurfunkprüfung");

        // menu bar
        let menu = gio::Menu::new();
        let menu_bar = gio::Menu::new();

        // define quit action
        let quit = gio::SimpleAction::new("quit", None);
        let window_clone = window.clone();
        quit.connect_activate(move |_, _| {
            window_clone.destroy();
        });
        app.add_action(&quit);

        menu.append("Quit", "app.quit");
        app.set_app_menu(&menu);
        app.set_menubar(&menu_bar);

        let datastore = DataStore::load(&std::path::PathBuf::from("/Users/pelsen/.config/afp/datastore.yml")).unwrap();
        mainview.init(&datastore);
        let datastore = Rc::new(Mutex::new(datastore));

        for (_i, section) in mainview.sections.borrow().iter().enumerate() {
            let mainview = mainview.clone();
            let datastore = datastore.clone();

            section.exam.connect_clicked(move |_widget| {
                datastore.lock().unwrap().section_mut(0).unwrap().question_mut(0).unwrap().answer(0);
                mainview.overview.update(&datastore.lock().unwrap());
            });
        }

        // position window and make visible
        window.add(&mainview.area);
        window.set_default_size(500, 400);
        window.set_position(gtk::WindowPosition::Center);
        window.show_all();
    }

    fn init(&self) {
        self.app.connect_startup(Self::startup);
        self.app.connect_shutdown(Self::shutdown);
        let mainview = self.main.clone();
        self.app.connect_activate(move |app| Self::activate(app, &mainview));
    }

    fn run(&self) {
        self.app.run(&env::args().collect::<Vec<_>>());
    }
}

fn main() {
    let app = App::new("net.xfbs.afs");
    app.init();
    app.run();
}

