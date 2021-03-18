use glow::HasContext;

pub trait SliceAsBytes<T> {
    fn as_mem_bytes(&self) -> &[u8];
}

impl<T: AsRef<[U]>, U> SliceAsBytes<U> for T {
    fn as_mem_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().as_ptr() as *const u8,
                std::mem::size_of::<T>() * self.as_ref().len(),
            )
        }
    }
}

pub trait Renderer {
    unsafe fn init(gl: &glow::Context) -> Self;

    unsafe fn update(&mut self, gl: &glow::Context);

    unsafe fn cleanup(&mut self, gl: &glow::Context);

    unsafe fn resize(&mut self, gl: &glow::Context, x: i32, y: i32) {
        gl.viewport(0, 0, x, y);
    }
}

pub fn run_program<Render: Renderer + 'static>() {
    unsafe {
        // Create a context from a WebGL2 context on wasm32 targets
        #[cfg(wasm)]
        let gl = {
            use wasm_bindgen::JsCast;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            let webgl2_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .unwrap();
            let gl = glow::Context::from_webgl2_context(webgl2_context);
            gl
        };

        // Create a context from a glutin window on non-wasm32 targets
        #[cfg(not(wasm))]
        let (gl, window, event_loop) = {
            let event_loop = glutin::event_loop::EventLoop::new();
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("Me Learning OpenGL 2")
                .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
            let gl =
                glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            (gl, window, event_loop)
        };

        let mut renderer = Render::init(&gl);

        #[cfg(not(wasm))]
        {
            use glutin::event::{Event, WindowEvent};
            use glutin::event_loop::ControlFlow;

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;
                match event {
                    Event::LoopDestroyed => {
                        return;
                    }
                    Event::MainEventsCleared => {
                        window.window().request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        renderer.update(&gl);
                        window.swap_buffers().unwrap();
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::Resized(physical_size) => {
                            window.resize(*physical_size);
                            renderer.resize(
                                &gl,
                                physical_size.width as i32,
                                physical_size.height as i32,
                            );
                        }
                        WindowEvent::CloseRequested => {
                            renderer.cleanup(&gl);
                            *control_flow = ControlFlow::Exit
                        }
                        _ => (),
                    },
                    _ => (),
                }
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            renderer.update(&gl);
            gl.flush();
        }
    }
}

pub fn handle_shader_compile_errors(
    gl: &glow::Context,
    shader: <glow::Context as HasContext>::Shader,
) {
    unsafe {
        if !gl.get_shader_compile_status(shader) {
            panic!("Shader compile error: {}", gl.get_shader_info_log(shader));
        }
    }
}

pub fn handle_program_link_errors(
    gl: &glow::Context,
    program: <glow::Context as HasContext>::Program,
) {
    unsafe {
        if !gl.get_program_link_status(program) {
            panic!("Shader link error: {}", gl.get_program_info_log(program));
        }
    }
}
