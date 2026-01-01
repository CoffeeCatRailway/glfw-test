#![allow(non_snake_case)]
/* Based on https://github.com/bwasty/learn-opengl-rs/blob/master/src/shader.rs */

use cgmath::{Matrix, Matrix4, Vector2, Vector3, Vector4};
use gl;
use gl::types::*;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

pub struct Shader {
    pub id: u32,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertexPath: &str, fragmentPath: &str) -> Shader {
        let mut shader = Shader { id: 0 };
        unsafe {
            let vertex = shader.compileShader(vertexPath, gl::VERTEX_SHADER);
            let fragment = shader.compileShader(fragmentPath, gl::FRAGMENT_SHADER);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);

            let fragColor = CString::new("o_color").unwrap();
            gl::BindFragDataLocation(id, 0, fragColor.as_ptr());
            gl::LinkProgram(id);

            shader.checkLinkStatus(id);

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }
        shader
    }

    pub fn newWithGeometry(vertexPath: &str, geometryPath: &str, fragmentPath: &str) -> Shader {
        let mut shader = Shader { id: 0 };
        unsafe {
            let vertex = shader.compileShader(vertexPath, gl::VERTEX_SHADER);
            let geometry = shader.compileShader(geometryPath, gl::GEOMETRY_SHADER);
            let fragment = shader.compileShader(fragmentPath, gl::FRAGMENT_SHADER);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, geometry);
            gl::AttachShader(id, fragment);

            let fragColor = CString::new("o_color").unwrap();
            gl::BindFragDataLocation(id, 0, fragColor.as_ptr());
            gl::LinkProgram(id);

            shader.checkLinkStatus(id);

            gl::DeleteShader(vertex);
            gl::DeleteShader(geometry);
            gl::DeleteShader(fragment);
            shader.id = id;
        }
        shader
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }

    pub fn getAttribLocation(&self, name: &str) -> GLint {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::GetAttribLocation(self.id, c_str.as_ptr() as *const GLchar)
        }
    }

    // Uniforms
    pub fn setUniform1i(&self, name: &str, value: i32) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, c_str.as_ptr() as *const GLchar),
                value,
            );
        }
    }

    pub fn setUniform1ui(&self, name: &str, value: u32) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::Uniform1ui(gl::GetUniformLocation(self.id, c_str.as_ptr()), value);
        }
    }

    pub fn setUniform1f(&self, name: &str, value: f32) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.id, c_str.as_ptr()), value);
        }
    }

    pub fn setUniform2fv(&self, name: &str, value: &Vector2<f32>) {
        self.setUniform2f(name, value.x, value.y);
    }

    pub fn setUniform2f(&self, name: &str, x: f32, y: f32) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::Uniform2f(gl::GetUniformLocation(self.id, c_str.as_ptr()), x, y);
        }
    }

    pub fn setUniform3fv(&self, name: &str, value: &Vector3<f32>) {
        self.setUniform3f(name, value.x, value.y, value.z);
    }

    pub fn setUniform3f(&self, name: &str, x: f32, y: f32, z: f32) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::Uniform3f(gl::GetUniformLocation(self.id, c_str.as_ptr()), x, y, z);
        }
    }

    pub fn setUniform4fv(&self, name: &str, value: &Vector4<f32>) {
        self.setUniform4f(name, value.x, value.y, value.z, value.w);
    }

    pub fn setUniform4f(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::Uniform4f(gl::GetUniformLocation(self.id, c_str.as_ptr()), x, y, z, w);
        }
    }

    pub fn setMatrix4f(&self, name: &str, mat: &Matrix4<f32>) {
        unsafe {
            let c_str = CString::new(name.as_bytes()).unwrap();
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, c_str.as_ptr()),
                1,
                gl::FALSE,
                mat.as_ptr(),
            );
        }
    }

    // Utility
    unsafe fn compileShader(&self, path: &str, shaderType: GLenum) -> GLuint {
        let mut file = File::open(path).unwrap_or_else(|_| panic!("Failed to open {}", path));
        let mut code = String::new();
        file.read_to_string(&mut code)
            .expect("Failed to read vertex shader");
        let source = CString::new(code.as_bytes()).unwrap();

        unsafe {
            let shader = gl::CreateShader(shaderType);

            // Attempt compile
            gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // Get compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Check if failed
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "Shader compilation error:\n{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );
            } else {
                println!("Shader compilation successful: {}-{}", shader, path);
            }

            shader
        }
    }

    unsafe fn checkLinkStatus(&self, id: GLuint) {
        unsafe {
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

            // Check if failed
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
                println!(
                    "Shader linking error:\n{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ProgramInfoLog not valid utf8")
                );
            } else {
                println!("Shader linking successful: {}", id);
            }
        }
    }
}
