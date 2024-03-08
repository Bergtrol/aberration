use bevy::{
    ecs::{
        event::EventReader,
        system::{Query, Res, ResMut, Resource},
    },
    input::{
        mouse::{MouseButton, MouseMotion, MouseWheel},
        ButtonInput,
    },
    window::{CursorGrabMode, Window},
};

#[derive(Resource, Default)]
pub struct InputState {
    pub moving_camera: bool,
    pub delta_x: f32,
    pub delta_y: f32,
    pub delta_zoom: f32,
}

pub fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut input_state: ResMut<InputState>,
) {
    if input_state.moving_camera == true {
        for event in motion_evr.read() {
            input_state.delta_x = event.delta.x;
            input_state.delta_y = event.delta.y;
        }

        //decay the input
        input_state.delta_x *= 0.5;
        input_state.delta_y *= 0.5;
    } else {
        input_state.delta_x = 0.0;
        input_state.delta_y = 0.0;
        input_state.delta_zoom = 0.0;
    }

    // zooming should happen regardless of if we're grabbed or not
    for event in mouse_wheel_events.read() {
        input_state.delta_zoom = event.y;
    }

    input_state.delta_zoom *= 0.5;
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
