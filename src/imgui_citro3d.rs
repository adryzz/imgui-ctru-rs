use std::mem;

use citro3d_sys::{
    C3D_Init, C3D_RenderTarget, C3D_RenderTargetCreate, C3D_RenderTargetSetOutput, C3D_Tex,
    Tex3DS_Texture, C3D_DEFAULT_CMDBUF_SIZE, C3D_DEPTHTYPE, GX_TRANSFER_FLIP_VERT,
    GX_TRANSFER_IN_FORMAT, GX_TRANSFER_OUT_FORMAT, GX_TRANSFER_OUT_TILED, GX_TRANSFER_RAW_COPY,
    GX_TRANSFER_SCALING,
};
use ctru_sys::{
    GFX_BOTTOM, GFX_LEFT, GFX_TOP, GPU_RB_DEPTH24_STENCIL8, GPU_RB_RGBA8, GX_TRANSFER_FMT_RGB8,
    GX_TRANSFER_FMT_RGBA8, DVLB_ParseFile, shaderProgramInit, shaderProgram_s, shaderProgramSetVsh,
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

struct Renderer {
    top_target: C3D_RenderTarget,
    bot_target: C3D_RenderTarget,
    gfx_texture_atlas: C3D_Tex,
    gfx_texture: Tex3DS_Texture,
}

impl Renderer {
    pub fn new(imgui: &mut Context) -> Self {
        unsafe {
            // Initialize Citro3D
            C3D_Init(2 * C3D_DEFAULT_CMDBUF_SIZE as usize);

            // Create render targets and bind them to the respective display
            let top = C3D_RenderTargetCreate(
                (FB_HEIGHT * 0.5) as i32,
                FB_WIDTH as i32,
                GPU_RB_RGBA8,
                depth_type(GPU_RB_DEPTH24_STENCIL8),
            );
            C3D_RenderTargetSetOutput(top, GFX_TOP, GFX_LEFT, display_transfer_flags());

            let bot = C3D_RenderTargetCreate(
                (FB_HEIGHT * 0.5) as i32,
                (FB_WIDTH * 0.8) as i32,
                GPU_RB_RGBA8,
                depth_type(GPU_RB_DEPTH24_STENCIL8),
            );
            C3D_RenderTargetSetOutput(bot, GFX_BOTTOM, GFX_LEFT, display_transfer_flags());

            imgui.set_renderer_name("Citro3D".to_string());


            let mut io = imgui.io_mut();
            io.backend_flags |= BackendFlags::RENDERER_HAS_VTX_OFFSET;
        
            let shbin = include_bytes!(env!("VSHADER_BIN_PATH"));


            let vsh = DVLB_ParseFile(shbin.as_ptr(), shbin.len());

            let s_program: shaderProgram_s = mem::zeroed();
            shaderProgramInit(s_program);
            shaderProgramSetVsh(s_program.mut_ptr(), vsh);
        }

        todo!()
    }
}