use std::time::{Duration, Instant};

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
    last_frame: Instant,
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

        Self {
            context,
            hid,
            gfx,
            last_frame: Instant::now(),
        }
    }

    pub fn prepare_frame(&mut self, apt: &mut Apt) -> &mut Ui {
        self.hid.scan_input();
        let io = self.context.io_mut();

        // set time delta
        let now = Instant::now();
        io.update_delta_time(now - self.last_frame);
        self.last_frame = now;

        imgui_ctru::update_touch(self.hid, io);
        imgui_ctru::update_gamepads(self.hid, io);
        imgui_ctru::update_keyboard(self.context, apt, self.gfx);

        self.context.new_frame()
    }

    pub fn prepare_render(&mut self, ui: &Ui) {
        let data = self.context.render();
        // TODO: renderer

        self.gfx.wait_for_vblank();
    }
}
