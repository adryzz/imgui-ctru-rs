use std::time::Duration;
use ctru::applets::swkbd::{Kind, Button, SoftwareKeyboard};
use ctru::prelude::KeyPad;
use ctru::services::hid::Hid;
use imgui::{BackendFlags, ConfigFlags, Context, Io, Key, MouseButton};

pub fn init(imgui: &mut Context) {

    // turn off filesystem stuff
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    let io = imgui.io_mut();

    // configure input devices
    io.config_flags |= ConfigFlags::IS_TOUCH_SCREEN;
    io.config_flags |= ConfigFlags::NAV_ENABLE_GAMEPAD;
    io.backend_flags |= BackendFlags::HAS_GAMEPAD;
    io.mouse_draw_cursor = false;

    imgui.set_platform_name("3DS".to_string());

    let style = imgui.style_mut();

    // turn off window rounding
    style.window_rounding = 0.0;
}

pub fn new_frame(imgui: &mut Context, hid: &mut Hid, delta_time: Duration) {
    let io = imgui.io_mut();
    // set time delta
    io.update_delta_time(delta_time);

    update_touch(hid, io);
    update_gamepads(hid, io);
    //update_keyboard(io);
}

pub fn update_touch(hid: &mut Hid, io: &mut Io) {
    if hid.keys_up().contains(KeyPad::TOUCH) {
        // keep mouse position for one frame for release event
        io.add_mouse_button_event(MouseButton::Left, false);
        return;
    }

    if !(hid.keys_held().contains(KeyPad::TOUCH)) {
        // set mouse cursor off-screen
        io.add_mouse_pos_event([-10.0, -10.0]);
        io.add_mouse_button_event(MouseButton::Left, false);
        return;
    }

    let pos = hid.touch_position();

    // transform position to bottom-screen space
    io.add_mouse_pos_event([pos.0 as f32 + 40.0, pos.1 as f32 + 240.0]);
    io.add_mouse_button_event(MouseButton::Left, true);
}

pub fn update_gamepads(hid: &mut Hid, io: &mut Io) {
    const MAPPING: [(KeyPad, Key); 12] = [
        (KeyPad::A, Key::GamepadFaceDown),
        (KeyPad::B, Key::GamepadFaceRight),
        (KeyPad::X, Key::GamepadFaceUp),
        (KeyPad::Y, Key::GamepadFaceLeft),
        (KeyPad::L, Key::GamepadL1),
        (KeyPad::ZL, Key::GamepadL1),
        (KeyPad::R, Key::GamepadR1),
        (KeyPad::ZR, Key::GamepadR1),
        (KeyPad::DPAD_UP, Key::GamepadDpadUp),
        (KeyPad::DPAD_RIGHT, Key::GamepadDpadRight),
        (KeyPad::DPAD_DOWN, Key::GamepadDpadDown),
        (KeyPad::DPAD_LEFT, Key::GamepadDpadLeft),
    ];

    // read buttons from 3DS
    let down = hid.keys_down();
    let up = hid.keys_up();

    for pair in MAPPING {
        if up.contains(pair.0) {
            io.add_key_event(pair.1, false);
        } else if down.contains(pair.0) {
            io.add_key_event(pair.1, true);
        }
    }

    // update joystick
    let analog = hid.circlepad_position();

    // deadzone stuff
    let analog_mapping: [(i16, Key, f32, f32); 4] = [
        (analog.0, Key::GamepadLStickLeft, -0.3, -0.9),
        (analog.0, Key::GamepadLStickRight, 0.3, 0.9),
        (analog.1, Key::GamepadLStickUp, 0.3, 0.9),
        (analog.1, Key::GamepadLStickDown, -0.3, -0.9)
    ];

    for pair in analog_mapping {
        let value = ((pair.0 as f32 / 156.0 - pair.2) / (pair.3 - pair.2)).clamp(0.0, 1.0);
        // add key analog event
        io.add_key_analog_event(pair.1, value > 0.1, value);
    }
}

pub fn update_keyboard(imgui: &mut Context) {
    let mut kbd = SoftwareKeyboard::new(Kind::Normal, 2);
    kbd.configure_button(Button::Left, "Cancel", false);
    kbd.configure_button(Button::Right, "OK", true);
    // todo: set keyboard initial text when ctru-rs supports it
    todo!()
}