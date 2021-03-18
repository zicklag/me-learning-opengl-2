use std::mem::size_of;

use glam::*;
use glow::HasContext;
use me_learning_opengl_2::*;

#[rustfmt::skip]
const QUAD_VERTS: &[f32] = &[
    // positions   // colors
    -0.05,  0.05,  1.0, 0.0, 0.0,
     0.05, -0.05,  0.0, 1.0, 0.0,
    -0.05, -0.05,  0.0, 0.0, 1.0,

    -0.05,  0.05,  1.0, 0.0, 0.0,
     0.05, -0.05,  0.0, 1.0, 0.0,   
     0.05,  0.05,  0.0, 1.0, 1.0	
];

struct Instancing {
    program: <glow::Context as HasContext>::Program,
    vao: <glow::Context as HasContext>::VertexArray,
    vbo: <glow::Context as HasContext>::Buffer,
}

impl Renderer for Instancing {
    unsafe fn init(gl: &glow::Context) -> Self {
        // Create offsets for the quads
        let mut offsets = Vec::with_capacity(100);
        let offset = 0.1;
        for y in (-10..10).step_by(2) {
            for x in (-10..10).step_by(2) {
                offsets.push(Vec2::new(
                    x as f32 / 10.0 + offset,
                    y as f32 / 10.0 + offset,
                ));
            }
        }

        let program = gl.create_program().unwrap();

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, include_str!("./instancing01/shader.vert"));
        gl.compile_shader(vertex_shader);
        handle_shader_compile_errors(gl, vertex_shader);

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, include_str!("./instancing01/shader.frag"));
        gl.compile_shader(fragment_shader);
        handle_shader_compile_errors(gl, fragment_shader);

        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);
        handle_program_link_errors(gl, program);

        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        gl.use_program(Some(program));

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));

        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            QUAD_VERTS.as_mem_bytes(),
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 5 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            5 * size_of::<f32>() as i32,
            2 * size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(0);
        gl.enable_vertex_attrib_array(1);

        let offset_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(offset_buffer));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            &offsets.as_mem_bytes(),
            glow::STATIC_DRAW,
        );
        gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 2 * size_of::<f32>() as i32, 0);
        gl.vertex_attrib_divisor(2, 1);
        gl.enable_vertex_attrib_array(2);

        Self { program, vao, vbo }
    }

    unsafe fn update(&mut self, gl: &glow::Context) {
        gl.clear_color(0.2, 0.2, 0.3, 1.0);
        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.draw_arrays_instanced(glow::TRIANGLES, 0, 6, 100);
    }

    unsafe fn cleanup(&mut self, gl: &glow::Context) {
        gl.delete_program(self.program);
        gl.delete_vertex_array(self.vao);
        gl.delete_buffer(self.vbo);
    }
}

fn main() {
    run_program::<Instancing>();
}
