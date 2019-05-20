mod main_controller;
mod overview_controller;
mod practise_controller;
mod section_controller;
mod section_overview_controller;

pub use main_controller::*;
pub use overview_controller::*;
pub use practise_controller::*;
pub use section_controller::*;
pub use section_overview_controller::*;

pub trait Controller {
    fn startup(&self);
    fn activate(&self);
    fn shutdown(&self);
}
