mod simulation;
mod status_bar;

pub use simulation::*;
pub use status_bar::*;

use bevy_egui::egui::{Button, Response, RichText, Ui};

pub fn create_header(ui: &mut Ui, title: &str) -> Response {
    let button = Button::new(RichText::new(title).heading())
        .min_size(bevy_egui::egui::Vec2::new(400.0, 0.0));
    ui.add(button)
}
