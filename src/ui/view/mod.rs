mod main;
mod overview;
mod practise;
mod section;
mod section_overview;

pub use main::*;
pub use overview::*;
pub use practise::*;
pub use section::*;
pub use section_overview::*;

pub trait View {
    fn widget(&self) -> gtk::Widget;
}

pub trait Labeled {
    fn label(&self) -> gtk::Label;
}
