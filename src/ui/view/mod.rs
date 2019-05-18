mod main_view;
mod over_view;
mod section_view;
mod question_view;

pub use main_view::*;
pub use over_view::*;
pub use section_view::*;
pub use question_view::*;

pub trait View {
    fn widget(&self) -> gtk::Widget;
}

pub trait Labeled {
    fn label(&self) -> gtk::Label;
}
