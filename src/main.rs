#![allow(non_snake_case)]

mod shader;

// extern crate glfw;
use glfw::{Context, Key, Action};

// extern crate gl;
use gl;
use gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::{CStr};
use std::{mem, ptr};
use std::os::raw::c_void;
use crate::shader::Shader;

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

// const VS_SOURCE: &'static str = r#"
// #version 330 core
//
// layout (location = 0) in vec3 i_pos;
//
// out vec2 v_pos;
//
// void main() {
// 	gl_Position = vec4(i_pos, 1.);
// 	v_pos = i_pos.xy;
// }
// "#;
//
// const FS_SOURCE: &'static str = r#"
// #version 330 core
//
// in vec2 v_pos;
// uniform vec3 u_color;
//
// out vec4 FragColor;
//
// void main() {
// 	// FragColor = vec4(1., .5, .2, 1.);
// 	vec3 posColor = vec3(v_pos * .5 + .5, 0.);
// 	FragColor = vec4((posColor + u_color) * .5, 1.);
// }
// "#;

// fn compileShader(source: &str, shaderType: GLenum) -> GLuint {
// 	unsafe {
// 		let shader = gl::CreateShader(shaderType);
//
// 		// Attempt compile
// 		let sourceStr = CString::new(source).unwrap();
// 		gl::ShaderSource(shader, 1, &sourceStr.as_ptr(), ptr::null());
// 		gl::CompileShader(shader);
//
// 		// Get compile status
// 		let mut status = gl::FALSE as GLint;
// 		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
//
// 		// Check if failed
// 		if status != (gl::TRUE as GLint) {
// 			let mut len = 0;
// 			gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
// 			let mut buf = Vec::with_capacity(len as usize);
// 			buf.set_len((len as usize) - 1);
// 			gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
// 			panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
// 		}
//
// 		shader
// 	}
// }
//
// fn linkShaderProgram(vs: GLuint, fs: GLuint) -> GLuint {
// 	unsafe {
// 		let program = gl::CreateProgram();
// 		gl::AttachShader(program, vs);
// 		gl::AttachShader(program, fs);
// 		gl::LinkProgram(program);
//
// 		// Get link status
// 		let mut status = gl::FALSE as GLint;
// 		gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
//
// 		// Check if failed
// 		if status != (gl::TRUE as GLint) {
// 			let mut len: GLint = 0;
// 			gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
// 			let mut buf = Vec::with_capacity(len as usize);
// 			buf.set_len((len as usize) - 1);
// 			gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
// 			panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
// 		}
// 		program
// 	}
// }

fn main() {
    println!("Hello, world!");

	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
	glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
	#[cfg(target_os = "macos")] // Whoever uses mac for some reason
	glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

	let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "Hello, world!", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.make_current();
	window.set_key_polling(true);
	window.set_framebuffer_size_polling(true);

	// gl: load all OpenGL function pointers
	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

	let (shader, VAO, VBO, EBO, elementCount) = unsafe {
		let shader = Shader::new("resources/shaders/vertex.vert", "resources/shaders/fragment.frag");
		// let vs = compileShader(VS_SOURCE, gl::VERTEX_SHADER);
		// let fs = compileShader(FS_SOURCE, gl::FRAGMENT_SHADER);
		// let shaderProgram = linkShaderProgram(vs, fs);
		// gl::DeleteShader(vs);
		// gl::DeleteShader(fs);

		// vertex data and vao
		// let vertices: [f32; 12] = [
		// 	 0.5,  0.5,  0.0, // top right
		// 	 0.5, -0.5,  0.0, // bottom right
		// 	-0.5, -0.5, -0.5, // bottom left
		// 	-0.5,  0.5,  0.0, // top left
		// ];
		// let indices = [
		// 	0, 1, 3,
		// 	1, 2, 3
		// ];
		let vertices: [f32; 24] = [
			-0.5,  0.5,  0.5, // top left
			 0.0,  0.5,  0.0, // top

			 0.5,  0.5,  0.5, // top right
			 0.5,  0.0,  0.0, // right

			 0.5, -0.5,  0.5, // bottom right
			 0.0, -0.5,  0.0, // bottom

			-0.5, -0.5,  0.5, // bottom left
			-0.5,  0.0,  0.0, // left
		];
		let indices = [
			0, 1, 7, // top left
			1, 2, 3, // top right
			3, 4, 5, // bottom right
			7, 5, 6, // bottom left
			1, 3, 5,
			1, 5, 7,
		];
		let (mut VAO, mut VBO, mut EBO) = (0, 0, 0);
		gl::GenVertexArrays(1, &mut VAO);
		gl::GenBuffers(1, &mut VBO);
		gl::GenBuffers(1, &mut EBO);
		gl::BindVertexArray(VAO);

		gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
		gl::BufferData(gl::ARRAY_BUFFER,
					   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
					   vertices.as_ptr() as *const c_void,
					   gl::STATIC_DRAW);

		gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
		gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
					   (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
					   indices.as_ptr() as *const c_void,
					   gl::STATIC_DRAW);

		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
		gl::EnableVertexAttribArray(0);

		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);

		(shader, VAO, VBO, EBO, indices.len() as GLsizei)
	};

	while !window.should_close() {
		// events
		handle_window_event(&mut window, &events);

		// render
		unsafe {
			gl::ClearColor(0.2, 0.3, 0.3, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);

			shader.bind();

			let time = glfw.get_time() as f32;

			let third = 1.0f32 / 3.0;
			// abs(mod(x,2)-1) = triangle wave

			let red = ((0.5 * time) % 2.0 - 1.0).abs();//time.cos() * 0.5 + 0.5;
			let green = ((0.5 * time + third) % 2.0 - 1.0).abs();//time.sin() * 0.5 + 0.5;
			let blue = ((0.5 * time - third) % 2.0 - 1.0).abs();
			shader.setUniform3f(c_str!("u_color"), red, green, blue);

			gl::BindVertexArray(VAO);
			// gl::DrawArrays(gl::TRIANGLES, 0, 3);
			// gl::BindVertexArray(0);
			gl::DrawElements(gl::TRIANGLES, elementCount, gl::UNSIGNED_INT, ptr::null());
		}

		window.swap_buffers();
		glfw.poll_events();
	}

	unsafe {
		shader.delete();
		gl::DeleteBuffers(1, &EBO);
		gl::DeleteBuffers(1, &VBO);
		gl::DeleteVertexArrays(1, &VAO);
	}
}

fn handle_window_event(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
	for (_, event) in glfw::flush_messages(events) {
		match event {
			glfw::WindowEvent::FramebufferSize(width, height) => {
				// make sure the viewport matches the new window dimensions; note that width and
				// height will be significantly larger than specified on retina displays.
				unsafe { gl::Viewport(0, 0, width, height) }
			}
			glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
				window.set_should_close(true)
			}
			_ => {}
		}
	}
}
