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

pub trait Controller {
    fn startup(&self);
    fn activate(&self);
    fn shutdown(&self);
}
