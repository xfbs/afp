mod main_controller;
mod section_controller;
mod overview_controller;

pub use main_controller::*;
pub use section_controller::*;
pub use overview_controller::*;

pub trait Controller {
    fn activate(&self);
    fn shutdown(&self);

}
