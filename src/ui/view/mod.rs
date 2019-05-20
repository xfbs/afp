mod main;
mod overview;
mod section;
mod section_overview;
mod practise;

pub use main::*;
pub use overview::*;
pub use section::*;
pub use practise::*;
pub use section_overview::*;

pub trait View {
    fn widget(&self) -> gtk::Widget;
}

pub trait Labeled {
    fn label(&self) -> gtk::Label;
}
