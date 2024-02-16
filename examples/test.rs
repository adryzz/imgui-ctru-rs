use imgui::*;
use imgui_ctru_rs::CtrPlatform;

fn main() {
    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");

    let mut context = Context::create();
    let platform = CtrPlatform::init(&mut context, hid, gfx);

    while apt.main_loop() {
        let ui = platform.prepare_frame(apt);
        // do stuff

        platform.prepare_render(ui);
    }
}
