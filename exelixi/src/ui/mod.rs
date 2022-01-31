use crate::prelude::*;

mod fov_viewer;
mod selection;
mod user_selection;
use bevy::sprite::Material2dPlugin;
pub use fov_viewer::*;
pub use selection::*;
pub use user_selection::*;

pub const UI_STATUS_BAR_HEIGHT: f32 = 120.0;
//
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<FovViewerMaterial>::default())
            //.add_system(debug_ui)
            .add_system(status_bar_ui)
            .add_system(user_selection)
            .add_system(spawn_fov_viewer_on_selected)
            .add_system_to_stage(CoreStage::PostUpdate, despawn_fov_viewer_on_deselected)
            .add_system_to_stage(CoreStage::PostUpdate, selection_changed);
    }
}

pub fn _debug_ui(
    egui_ctx: Res<EguiContext>,
    selection: Query<(&Locomotion, &Body), With<Selected>>,
    simulation: Res<Simulation>,
    config: Res<SimulationConfig>,
    diagnostics: Res<Diagnostics>,
) {
    egui::Window::new("Debug")
        .vscroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Simulation");
            ui.label(format!("width : {}", config.environment.width));
            ui.label(format!("height: {}", config.environment.height));
            if let Ok((locomotion, body)) = selection.get_single() {
                ui.heading("Selection");
                ui.label(format!("linear: {:.1}", locomotion.linear));
                ui.label(format!("angular: {:.1}", locomotion.angular));
                ui.label(format!("energy: {:.1}", body.energy()));
            }
            ui.separator();

            let mut fps_s = "N/A".to_string();
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    fps_s = format!("{:.1}", average);
                }
            }
            ui.label(format!("fps: {}", fps_s));
            ui.label(format!("sts: {:.2}", simulation.sts(&config)));
        });
}
pub fn status_bar_ui(
    egui_ctx: Res<EguiContext>,
    mut simulation: ResMut<Simulation>,
    diagnostics: Res<Diagnostics>,
) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.family_and_size.insert(
        egui::TextStyle::Body,
        (egui::FontFamily::Proportional, 20.0),
    );
    fonts.family_and_size.insert(
        egui::TextStyle::Monospace,
        (egui::FontFamily::Proportional, 20.0),
    );
    fonts.family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 30.0),
    );
    egui_ctx.ctx().set_fonts(fonts);

    let size_color = egui::Color32::from_rgb(51, 51, 230);
    let dead_color = egui::Color32::from_rgb(230, 0, 0);
    let avg_color = egui::Color32::from_rgb(25, 180, 25);
    let population_size_line = egui::plot::Line::new(egui::plot::Values::from_values_iter(
        simulation
            .statistics
            .generations
            .iter()
            .enumerate()
            .map(|(i, s)| egui::plot::Value::new(i as f64, s.start_size as f64)),
    ))
    .color(size_color);
    let population_dead_line = egui::plot::Line::new(egui::plot::Values::from_values_iter(
        simulation
            .statistics
            .generations
            .iter()
            .enumerate()
            .map(|(i, s)| egui::plot::Value::new(i as f64, (s.start_size - s.end_size) as f64)),
    ))
    .color(dead_color);
    let plot_bottom = egui::plot::Line::new(egui::plot::Values::from_values_iter(
        simulation
            .statistics
            .generations
            .iter()
            .enumerate()
            .map(|(i, s)| egui::plot::Value::new(i as f64, s.food_decay as f64)),
    ))
    .color(avg_color);
    egui::TopBottomPanel::bottom("bottom_panel")
        .frame(
            egui::Frame::default()
                .fill(egui::Color32::from_rgb(30, 30, 30))
                .margin(egui::Vec2::new(10.0, 10.0)),
        )
        .max_height(UI_STATUS_BAR_HEIGHT)
        .show(egui_ctx.ctx(), |ui| {
            let half_width = ui.available_width() / 2.0;
            let mut spacing = ui.spacing_mut();
            spacing.button_padding = egui::Vec2::new(2.0, 2.0);
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new(format!("generation : {:3}", simulation.generation))
                        .text_style(egui::TextStyle::Monospace),
                );
                ui.label(
                    egui::RichText::new(format!("step : {:4}", simulation.steps))
                        .text_style(egui::TextStyle::Monospace),
                );
                let mut fps_s = "N/A".to_string();
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        fps_s = format!("{:.1}", average);
                    }
                }
                ui.label(format!("fps: {}", fps_s));
                ui.add_space(ui.available_width() - half_width - 30.0 * 2.5);
                // ⚙
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏹").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Stop")
                    .clicked()
                {
                    simulation.control.speed_factor = 1;
                    simulation.control.state = SimulationControlState::Paused;
                }
                let (text, hover_text, control) =
                    if simulation.control.state == SimulationControlState::Paused {
                        ("▶", "Run", SimulationControlState::Run)
                    } else {
                        ("⏸", "Pause", SimulationControlState::Paused)
                    };
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new(text).color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text(hover_text)
                    .clicked()
                {
                    simulation.control.state = control;
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬇").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Decrease speed")
                    .clicked()
                {
                    simulation.control.speed_factor = (simulation.control.speed_factor / 2).max(1);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⬆").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Increase speed")
                    .clicked()
                {
                    simulation.control.speed_factor *= 2;
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⏭").color(egui::Color32::from_rgb(0, 255, 0)),
                    ))
                    .on_hover_text("Fastest")
                    .clicked()
                {
                    simulation.control.state = SimulationControlState::Fastest;
                }
                ui.add_space(half_width / 6.0);
                ui.vertical(|ui| {
                    let plot = egui::plot::Plot::new("population_plot")
                        .height(50.0)
                        .width(half_width / 1.7)
                        .show_x(false)
                        .show_y(true)
                        .center_x_axis(false)
                        .center_y_axis(false)
                        .allow_zoom(false)
                        .allow_drag(false);
                    plot.show(ui, |plot_ui| {
                        plot_ui.line(population_dead_line);
                        plot_ui.line(population_size_line);
                    });
                    let plot = egui::plot::Plot::new("bottom_plot")
                        .height(50.0)
                        .width(half_width / 1.7)
                        .show_x(false)
                        .show_y(true)
                        .center_x_axis(false)
                        .center_y_axis(false)
                        .allow_zoom(false)
                        .allow_drag(false);
                    plot.show(ui, |plot_ui| {
                        plot_ui.line(plot_bottom);
                    });
                });
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "{}",
                            simulation.statistics.latest_start_size()
                        ))
                        .color(size_color),
                    )
                    .on_hover_text("Starting number of animals");
                    ui.add_space(25.0 - 8.0);
                    ui.label(
                        egui::RichText::new(format!("{}", simulation.statistics.latest_dead()))
                            .color(dead_color),
                    )
                    .on_hover_text("Number of dead animals");
                    ui.add_space(25.0 - 8.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "{:.1}",
                            simulation.statistics.latest_food_decay()
                        ))
                        .color(avg_color),
                    )
                    .on_hover_text("Number food decayed");
                });
            });
        });
}
