#![allow(non_snake_case)]

use crate::shader::Shader;
use gl::types::{GLsizei, GLsizeiptr, GLuint};
use std::os::raw::c_void;

type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

pub struct LineRenderer {
    vec: Vec<f32>,
    shader: Shader,
    vao: GLuint,
    vbo: GLuint,
    floatsPushed: usize,
    lastFloatsPushed: usize,
    pub enabled: bool,
}

/*
 * Shader data:
 * - float3 pos1
 * - float3 color1
 * - float3 pos2
 * - float3 color2
 *
 * Floats: 6
 * Bytes: 48
 */
const FLOATS: usize = 6;
const FLOAT_SIZE: usize = size_of::<f32>();
const SHADER_VERT: &str = "resources/shaders/line_renderer.vert";
const SHADER_FRAG: &str = "resources/shaders/line_renderer.frag";

impl LineRenderer {
    pub fn new(capacity: usize) -> LineRenderer {
        let mut renderer = LineRenderer {
            vec: Vec::with_capacity(capacity),
            shader: Shader::new(SHADER_VERT, SHADER_FRAG),
            vao: 0,
            vbo: 0,
            floatsPushed: 0,
            lastFloatsPushed: capacity,
            enabled: true,
        };
        unsafe {
            gl::CreateVertexArrays(1, &mut renderer.vao);
            gl::CreateBuffers(1, &mut renderer.vbo);
            gl::BindVertexArray(renderer.vao);

            gl::NamedBufferData(
                renderer.vbo,
                (capacity * FLOAT_SIZE) as GLsizeiptr,
                renderer.vec.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );
            gl::VertexArrayVertexBuffer(
                renderer.vao,
                0,
                renderer.vbo,
                0,
                (FLOATS * FLOAT_SIZE) as GLsizei,
            );

            let locPos = renderer.shader.getAttribLocation("i_position") as GLuint;
            let locCol = renderer.shader.getAttribLocation("i_color") as GLuint;

            let mut offset: GLuint = 0;
            gl::VertexArrayAttribFormat(
                renderer.vao,
                locPos,
                3,
                gl::FLOAT,
                gl::FALSE,
                offset,
            );
            gl::VertexArrayAttribBinding(renderer.vao, locPos, 0);
            offset += 3 * FLOAT_SIZE as GLuint;

            gl::VertexArrayAttribFormat(
                renderer.vao,
                locCol,
                3,
                gl::FLOAT,
                gl::FALSE,
                offset,
            );
            gl::VertexArrayAttribBinding(renderer.vao, locCol, 0);
            // offset += 3 * FLOAT_SIZE as GLuint;

            gl::EnableVertexArrayAttrib(renderer.vao, locPos);
            gl::EnableVertexArrayAttrib(renderer.vao, locCol);
            renderer.vec.clear();
        }
        renderer
    }

    pub fn pushLine(&mut self, pos1: Vector3, color1: Vector3, pos2: Vector3, color2: Vector3) {
        if !self.enabled {
            return;
        }
        self.vec.push(pos1.x);
        self.vec.push(pos1.y);
        self.vec.push(pos1.z);
        self.vec.push(color1.x);
        self.vec.push(color1.y);
        self.vec.push(color1.z);

        self.vec.push(pos2.x);
        self.vec.push(pos2.y);
        self.vec.push(pos2.z);
        self.vec.push(color2.x);
        self.vec.push(color2.y);
        self.vec.push(color2.z);

        self.floatsPushed += FLOATS * 2;
    }

    pub fn drawFlush(&mut self, pvMatrix: &Matrix4) {
        if self.vec.len() < FLOATS * 2 || self.floatsPushed < FLOATS * 2 {
            return;
        }

        self.shader.bind();
        self.shader.setMatrix4f("u_pvm", pvMatrix);

        unsafe {
            if self.floatsPushed > self.lastFloatsPushed {
                gl::NamedBufferData(
                    self.vbo,
                    (self.vec.len() * FLOAT_SIZE) as GLsizeiptr,
                    self.vec.as_ptr() as *const c_void,
                    gl::DYNAMIC_DRAW,
                );
            } else {
                gl::NamedBufferSubData(
                    self.vbo,
                    0,
                    (self.vec.len() * FLOAT_SIZE) as GLsizeiptr,
                    self.vec.as_ptr() as *const c_void,
                );
            }

            gl::BindVertexArray(self.vao);
            let drawCount = self.vec.len() / FLOATS;
            // println!("drawCount: {}", drawCount);
            gl::DrawArrays(gl::LINES, 0, drawCount as GLsizei);
        }

        self.vec.clear();
        self.lastFloatsPushed = self.floatsPushed;
        self.floatsPushed = 0;
    }

    pub fn destroy(&self) {
        println!("Destroying line renderer");
        self.shader.delete();
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
