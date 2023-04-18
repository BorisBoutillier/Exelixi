mod selection;
mod simulation;
mod status_bar;

pub use selection::*;
pub use simulation::*;
pub use status_bar::*;

use bevy_egui::egui::{Button, Color32, Response, RichText, Ui};

pub const HONEY: Color32 = Color32::from_rgb(0xEC, 0x97, 0x06);

pub fn create_header(ui: &mut Ui, title: &str) -> Response {
    let button = Button::new(RichText::new(title).heading())
        .min_size(bevy_egui::egui::Vec2::new(200.0, 0.0));
    ui.add(button)
}

pub const NEURON_COLOR_0: Color32 = Color32::from_rgb(0xFE, 0xF0, 0x01);
pub const NEURON_COLOR_1: Color32 = Color32::from_rgb(0xFF, 0xCE, 0x03);
pub const NEURON_COLOR_2: Color32 = Color32::from_rgb(0xFD, 0x9A, 0x01);
pub const NEURON_COLOR_3: Color32 = Color32::from_rgb(0xFD, 0x61, 0x04);
pub const NEURON_COLOR_4: Color32 = Color32::from_rgb(0xFF, 0x2C, 0x05);
pub const NEURON_COLOR_5: Color32 = Color32::from_rgb(0xF0, 0x05, 0x05);
pub fn get_text_for_neuron(value: f32) -> RichText {
    let text = RichText::new(format!("{:1.1}", value))
        .monospace()
        .size(10.0);
    let a = value.abs();
    if a < f32::EPSILON {
        text
    } else if a < 0.16 {
        text.color(NEURON_COLOR_0)
    } else if a < 0.32 {
        text.color(NEURON_COLOR_1)
    } else if a < 0.48 {
        text.color(NEURON_COLOR_2)
    } else if a < 0.64 {
        text.color(NEURON_COLOR_3)
    } else if a < 0.80 {
        text.color(NEURON_COLOR_4)
    } else {
        text.color(NEURON_COLOR_5)
    }
}
