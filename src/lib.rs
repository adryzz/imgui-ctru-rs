use std::time::Duration;

use ctru::services::apt::Apt;
use ctru::services::gfx::Gfx;
use ctru::services::hid::Hid;
use imgui::{Context, Ui, Window};

pub mod imgui_citro3d;
pub mod imgui_ctru;
use imgui::{BackendFlags, ConfigFlags};
pub struct CtrPlatform<'a> {
    context: &'a mut Context,
    hid: &'a mut Hid,
    gfx: &'a mut Gfx,
}

impl<'a> CtrPlatform<'a> {
    pub fn init(context: &'a mut Context, hid: &'a mut Hid, gfx: &'a mut Gfx) -> Self {
        // turn off filesystem stuff
        context.set_ini_filename(None);
        context.set_log_filename(None);
        let io = context.io_mut();

        // configure input devices
        io.config_flags |= ConfigFlags::IS_TOUCH_SCREEN;
        io.config_flags |= ConfigFlags::NAV_ENABLE_GAMEPAD;
        io.backend_flags |= BackendFlags::HAS_GAMEPAD;
        io.mouse_draw_cursor = false;

        context.set_platform_name("3DS".to_string());

        let style = context.style_mut();

        // turn off window rounding
        style.window_rounding = 0.0;

        Self { context, hid, gfx }
    }

    pub fn prepare_frame(&mut self, apt: &mut Apt, delta_time: Duration) {
        let io = self.context.io_mut();
        // set time delta
        io.update_delta_time(delta_time);

        imgui_ctru::update_touch(self.hid, io);
        imgui_ctru::update_gamepads(self.hid, io);
        imgui_ctru::update_keyboard(self.context, apt, self.gfx);
    }

    pub fn prepare_render(&mut self, ui: &Ui, window: &Window<&str>) {
        // TODO: renderer
    }
}
