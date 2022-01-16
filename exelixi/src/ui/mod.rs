use crate::prelude::*;

mod fov_viewer;
use bevy::sprite::Material2dPlugin;
pub use fov_viewer::*;

pub const UI_STATUS_BAR_HEIGHT: f32 = 120.0;
//
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<FovViewerMaterial>::default())
            .add_system(debug_ui)
            .add_system(status_bar_ui)
            .add_system(spawn_fov_viewer_on_selected)
            .add_system(despawn_fov_viewer_on_deselected);
    }
}

pub fn debug_ui(
    egui_ctx: Res<EguiContext>,
    selection: Query<(&Transform, &Velocity, &Stomach), With<Selected>>,
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
            ui.label(simulation.speed.to_string());
            ui.label("fitness".to_string());
            ui.label(format!(
                "    min: {:.2}",
                simulation.statistics.latest_min_fitness()
            ));
            ui.label(format!(
                "    max: {:.2}",
                simulation.statistics.latest_max_fitness()
            ));
            ui.label(format!(
                "    avg: {:.2}",
                simulation.statistics.latest_avg_fitness()
            ));
            if let Ok((transform, velocity, stomach)) = selection.get_single() {
                ui.heading("Selection");
                ui.label(format!("x: {:.1}", transform.translation.x));
                ui.label(format!("y: {:.1}", transform.translation.y));
                ui.label(format!("linear: {:.1}", velocity.linear));
                ui.label(format!("angular: {:.1}", velocity.angular));
                ui.label(format!("satiation: {:.1}", stomach.satiation));
                //ui.label(format!(
                //    "eye wall vision: {}",
                //    eye.process_vision_walls(transform, &config)
                //        .iter()
                //        .map(|f| format!("{:.1} ", f))
                //        .collect::<String>()
                //));
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
pub fn status_bar_ui(egui_ctx: Res<EguiContext>, mut simulation: ResMut<Simulation>) {
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
    let mut new_speed = None;
    let population_size_line = egui::plot::Line::new(egui::plot::Values::from_values_iter(
        simulation
            .statistics
            .population
            .iter()
            .enumerate()
            .map(|(i, s)| egui::plot::Value::new(i as f64, s.size() as f64)),
    ))
    .color(size_color);
    let population_dead_line = egui::plot::Line::new(egui::plot::Values::from_values_iter(
        simulation
            .statistics
            .population
            .iter()
            .enumerate()
            .map(|(i, s)| egui::plot::Value::new(i as f64, s.dead() as f64)),
    ))
    .color(dead_color);
    let population_avg_fitness_line = egui::plot::Line::new(egui::plot::Values::from_values_iter(
        simulation
            .statistics
            .population
            .iter()
            .enumerate()
            .map(|(i, s)| egui::plot::Value::new(i as f64, s.avg_fitness() as f64)),
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
                    egui::RichText::new(format!("step : {:4}", simulation.age))
                        .text_style(egui::TextStyle::Monospace),
                );
                ui.add_space(ui.available_width() - half_width - 30.0 * 2.5);
                for (text, hover_text, speed) in &[
                    ("⏸", "Pause", SimulationSpeed::Paused),
                    ("▶", "Normal Speed", SimulationSpeed::Normal),
                    ("⏩", "Fast Speed", SimulationSpeed::Fast),
                    ("⏭", "Maximum Speed", SimulationSpeed::Fastest),
                ] {
                    if ui
                        .add(egui::Button::new(
                            egui::RichText::new(*text).color(egui::Color32::from_rgb(0, 255, 0)),
                        ))
                        .on_hover_text(*hover_text)
                        .clicked()
                    {
                        new_speed = Some(*speed);
                    }
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("⚙").color(egui::Color32::from_rgb(220, 220, 220)),
                    ))
                    .on_hover_text("simulation configuration")
                    .clicked()
                {}
                ui.add_space(half_width / 5.0);
                ui.vertical(|ui| {
                    let plot = egui::plot::Plot::new("population_plot")
                        .height(50.0)
                        .width(half_width * 3.0 / 5.0)
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
                    let plot = egui::plot::Plot::new("fitness_plot")
                        .height(50.0)
                        .width(half_width * 3.0 / 5.0)
                        .show_x(false)
                        .show_y(true)
                        .center_x_axis(false)
                        .center_y_axis(false)
                        .allow_zoom(false)
                        .allow_drag(false);
                    plot.show(ui, |plot_ui| {
                        plot_ui.line(population_avg_fitness_line);
                    });
                });
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", simulation.statistics.latest_size()))
                            .color(size_color),
                    );
                    ui.add_space(25.0 - 8.0);
                    ui.label(
                        egui::RichText::new(format!("{}", simulation.statistics.latest_dead()))
                            .color(dead_color),
                    );
                    ui.add_space(25.0 - 8.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "{:.1}",
                            simulation.statistics.latest_avg_fitness()
                        ))
                        .color(avg_color),
                    );
                });
            });
        });
    if let Some(new_speed) = new_speed {
        simulation.speed = new_speed;
    }
}
