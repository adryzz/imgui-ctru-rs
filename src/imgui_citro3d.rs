use core::slice;
use std::mem;

use citro3d_sys::{
    C3D_Init, C3D_RenderTarget, C3D_RenderTargetCreate, C3D_RenderTargetSetOutput, C3D_Tex,
    Tex3DS_Texture, C3D_DEFAULT_CMDBUF_SIZE, C3D_DEPTHTYPE, GX_TRANSFER_FLIP_VERT,
    GX_TRANSFER_IN_FORMAT, GX_TRANSFER_OUT_FORMAT, GX_TRANSFER_OUT_TILED, GX_TRANSFER_RAW_COPY,
    GX_TRANSFER_SCALING,
};
use ctru_sys::{
    shaderInstanceGetUniformLocation, shaderProgramInit, shaderProgramSetVsh, shaderProgram_s,
    DVLB_ParseFile, DVLE_s, GFX_BOTTOM, GFX_LEFT, GFX_TOP, GPU_RB_DEPTH24_STENCIL8, GPU_RB_RGBA8,
    GX_TRANSFER_FMT_RGB8, GX_TRANSFER_FMT_RGBA8,
};
use imgui::{
    BackendFlags, ConfigFlags, Context, DrawCmd, DrawCmdParams, DrawData, Io, Key, MouseButton,
    TextureId, Textures,
};

const CLEAR_COLOR: u32 = 0x204B7AFF;
const SCREEN_WIDTH: f32 = 400.0;
const SCREEN_HEIGHT: f32 = 480.0;

#[cfg(ANTI_ALIAS)]
const FB_SCALE: f32 = 2.0;
#[cfg(ANTI_ALIAS)]
const TRANSFER_SCALING: u32 = ctru_sys::GX_TRANSFER_SCALE_XY;

#[cfg(not(ANTI_ALIAS))]
const FB_SCALE: f32 = 1.0;
#[cfg(not(ANTI_ALIAS))]
const TRANSFER_SCALING: u32 = ctru_sys::GX_TRANSFER_SCALE_NO;

const FB_WIDTH: f32 = SCREEN_WIDTH * FB_SCALE;

const FB_HEIGHT: f32 = SCREEN_HEIGHT * FB_SCALE;

fn display_transfer_flags() -> u32 {
    GX_TRANSFER_FLIP_VERT(false)
        | GX_TRANSFER_OUT_TILED(false)
        | GX_TRANSFER_RAW_COPY(false)
        | GX_TRANSFER_IN_FORMAT(GX_TRANSFER_FMT_RGBA8)
        | GX_TRANSFER_OUT_FORMAT(GX_TRANSFER_FMT_RGB8)
        | GX_TRANSFER_SCALING(TRANSFER_SCALING)
}

fn depth_type(depth: u32) -> C3D_DEPTHTYPE {
    C3D_DEPTHTYPE { __e: depth }
}

pub struct Renderer {
    top_target: *mut C3D_RenderTarget,
    bot_target: *mut C3D_RenderTarget,
    gfx_texture_atlas: C3D_Tex,
    gfx_texture: Tex3DS_Texture,
    s_proj_location: i8,
}

impl Renderer {
    pub fn new(imgui: &mut Context) -> Self {
        unsafe {
            // very very bad dont do this
            let mut renderer: Self = mem::zeroed();

            // Initialize Citro3D
            C3D_Init(2 * C3D_DEFAULT_CMDBUF_SIZE as usize);

            // Create render targets and bind them to the respective display
            renderer.top_target = C3D_RenderTargetCreate(
                (FB_HEIGHT * 0.5) as i32,
                FB_WIDTH as i32,
                GPU_RB_RGBA8,
                depth_type(GPU_RB_DEPTH24_STENCIL8),
            );
            C3D_RenderTargetSetOutput(
                renderer.top_target,
                GFX_TOP,
                GFX_LEFT,
                display_transfer_flags(),
            );

            renderer.bot_target = C3D_RenderTargetCreate(
                (FB_HEIGHT * 0.5) as i32,
                (FB_WIDTH * 0.8) as i32,
                GPU_RB_RGBA8,
                depth_type(GPU_RB_DEPTH24_STENCIL8),
            );
            C3D_RenderTargetSetOutput(
                renderer.bot_target,
                GFX_BOTTOM,
                GFX_LEFT,
                display_transfer_flags(),
            );

            imgui.set_renderer_name("Citro3D".to_string());

            let io = imgui.io_mut();
            io.backend_flags |= BackendFlags::RENDERER_HAS_VTX_OFFSET;

            let shbin = include_bytes!(env!("VSHADER_BIN_PATH"));

            let mut shbin_mut = shbin.clone();

            let a = convert(&mut shbin_mut[..]);

            let vsh = DVLB_ParseFile(a.as_mut_ptr(), a.len() as u32);

            let mut s_program: shaderProgram_s = mem::zeroed();
            let _ = shaderProgramInit(&mut s_program);
            let _ = shaderProgramSetVsh(&mut s_program, vsh as *mut DVLE_s);

            renderer.s_proj_location = shaderInstanceGetUniformLocation(
                s_program.vertexShader,
                std::ffi::CStr::from_bytes_with_nul(b"proj\0")
                    .unwrap()
                    .as_ptr(),
            );

            renderer
        }
    }
}

fn convert<'a>(data: &'a mut [u8]) -> &mut [u32] {
    if data.len() % 4 != 0 {
        panic!("Wrong size");
    }

    unsafe { slice::from_raw_parts_mut(data.as_mut_ptr() as *mut u32, data.len() / 4) }
}
