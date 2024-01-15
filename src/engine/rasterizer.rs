use std::ffi::CString;

use gl::types::GLint;
use gl_rs as gl;
use glutin::{
    config::Config,
    display::GetGlDisplay,
    prelude::{GlConfig, GlDisplay},
};
use rand::prelude::*;
use skia_safe::{
    gpu::{self, backend_render_targets, gl::FramebufferInfo, SurfaceOrigin},
    Color, Color4f, ColorSpace, ColorType, Paint, Point, Surface,
};

pub struct Rasterizer {
    width: i32,
    height: i32,
    surface: Surface,
    context: skia_safe::gpu::DirectContext,
}

impl Rasterizer {
    pub fn new(gl_config: &Config, width: i32, height: i32) -> Self {
        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });

        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .expect("Could not create interface");

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(Some(interface), None)
            .expect("Could not create direct context");

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        fn create_surface(
            (width, height): (i32, i32),
            fb_info: FramebufferInfo,
            gr_context: &mut skia_safe::gpu::DirectContext,
            num_samples: usize,
            stencil_size: usize,
        ) -> Surface {
            let backend_render_target = backend_render_targets::make_gl(
                (width, height),
                num_samples,
                stencil_size,
                fb_info,
            );

            gpu::surfaces::wrap_backend_render_target(
                gr_context,
                &backend_render_target,
                SurfaceOrigin::BottomLeft,
                ColorType::RGBA8888,
                Some(ColorSpace::new_srgb()),
                None,
            )
            .expect("Could not create skia surface")
        }
        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let surface = create_surface(
            (width, height),
            fb_info,
            &mut gr_context,
            num_samples,
            stencil_size,
        );

        Self {
            width,
            height,
            surface,
            context: gr_context,
        }
    }

    pub fn draw_surface(&mut self) {
        let paint = Paint::new(Color4f::from(0xff_ff0000), None);

        let mut rng = rand::thread_rng();
        let random_x: f32 = rng.gen::<f32>() * self.width as f32;
        let random_y: f32 = rng.gen::<f32>() * self.height as f32;

        let canvas = self.surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_circle(Point::new(random_x, random_y), 40.0, &paint);

        self.context.flush_and_submit();
    }
}
