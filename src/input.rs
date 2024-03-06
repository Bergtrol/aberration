use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, Res, ResMut, Resource},
    },
    input::{
        mouse::{MouseButton, MouseMotion},
        ButtonInput,
    },
    window::{CursorGrabMode, Window},
};

#[derive(Resource)]
pub struct InputState {
    pub moving_camera: bool,
}

pub fn mouse_motion(mut motion_evr: EventReader<MouseMotion>, input_state: Res<InputState>) {
    if input_state.moving_camera == true {
        for event in motion_evr.read() {
            println!(
                "Mouse moved: X: {} px, Y: {} px",
                event.delta.x, event.delta.y
            );
        }
    }
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut input_state: ResMut<InputState>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        input_state.moving_camera = true;
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if mouse.just_released(MouseButton::Left) {
        input_state.moving_camera = false;
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
