use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

#[derive(Default)]
struct UiState {
    show: bool,
}

fn ui_example(egui_context: ResMut<EguiContext>, ui_state: Res<UiState>) {
    if ui_state.show {
        egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
            ui.label("world");
        });
    }
}

fn ui_toggle(keys: Res<Input<KeyCode>>, mut ui_state: ResMut<UiState>) {
    if keys.pressed(KeyCode::LControl)
        && keys.pressed(KeyCode::LShift)
        && keys.just_pressed(KeyCode::D)
    {
        ui_state.show = !ui_state.show;
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_system(ui_toggle)
            .add_system(ui_example);
    }
}
