use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use engine::{Application, Mat4};

use crate::input_mapper::InputMapper;
use crate::render::context::GlContext;
use crate::render::shader::{FRAGMENT_SHADER, ShaderProgram, VERTEX_SHADER};
use crate::render::world_renderer::WorldRenderer;

pub struct AppHandler {
    app: Application,
    mapper: InputMapper,
    ctx: Option<GlContext>,
    program: Option<ShaderProgram>,
    renderer: WorldRenderer,
}

impl AppHandler {
    pub fn new(app: Application) -> Self {
        Self {
            app,
            mapper: InputMapper::new(),
            ctx: None,
            program: None,
            renderer: WorldRenderer::new(),
        }
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("Qubecat Engine");
        let ctx = GlContext::create(event_loop, attrs).expect("не удалось создать GL-контекст");

        self.program = Some(ShaderProgram::new(&ctx.gl, VERTEX_SHADER, FRAGMENT_SHADER));
        self.ctx = Some(ctx);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => self.app.engine.resources.input.request_quit(),
            WindowEvent::Resized(size) => {
                if let Some(ctx) = &mut self.ctx {
                    ctx.resize(size.width, size.height);
                }
                self.app.engine.resources.camera.aspect =
                    size.width as f32 / size.height.max(1) as f32;
            }
            WindowEvent::RedrawRequested => {
                let Some(ctx) = &mut self.ctx else {
                    return;
                };

                let view_proj: Mat4 = self.app.engine.resources.camera.view_projection();

                ctx.clear();
                if let Some(program) = &self.program {
                    self.renderer.render(&ctx.gl, program, &view_proj);
                }
                ctx.present();
            }
            ref ev => self
                .mapper
                .handle_event(ev, &mut self.app.engine.resources.input),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self
            .app
            .engine
            .resources
            .input
            .is_just_pressed(engine::input::Key::Escape)
        {
            self.app.engine.resources.input.request_quit();
        }

        if self.app.should_quit() {
            event_loop.exit();
            return;
        }

        self.app.step();

        // Синхронизация мешей с GPU после тиков, до рендера кадра.
        if let (Some(ctx), Some(_)) = (&mut self.ctx, &self.program) {
            let voxels = &mut self.app.engine.resources.voxels;
            self.renderer.sync(&ctx.gl, voxels);
        }

        if let Some(ctx) = &self.ctx {
            ctx.window().request_redraw();
        }

        self.app.engine.resources.input.begin_frame();
    }
}
