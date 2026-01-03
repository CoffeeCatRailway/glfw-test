#![allow(non_snake_case)]

mod shader;
mod camera;
mod line_renderer;

use crate::shader::Shader;
use crate::camera::{Camera, Movement};
use crate::line_renderer::LineRenderer;

use imgui::Context as ImContext;
use imgui_glfw_rs::ImguiGLFW;
use imgui_glfw_rs::glfw::{Action, Context, Key};
use imgui_glfw_rs::imgui as ImGui;

use gl;
use gl::types::*;

use std::os::raw::c_void;
use std::ptr;
use std::f32::consts::PI;
use cgmath::{perspective, vec3, Array, Deg, Matrix4, Point3, SquareMatrix, Vector3};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    println!("Hello, world!");

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")] // Whoever uses mac for some reason
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
	println!("GLFW initialized, version: {}", glfw::get_version_string());

	let (mut winWidth, mut winHeight) = (SCR_WIDTH, SCR_HEIGHT);
    let (mut window, events) = glfw
        .create_window(
			winWidth,
			winHeight,
            "Hello, world!",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_all_polling(true);
	
	window.set_cursor_mode(glfw::CursorMode::Normal);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
	println!("OpenGL function pointers");

	let mut imgui = ImContext::create();
	imgui.set_ini_filename(None);
	imgui.set_log_filename(None);
	let mut imguiGlfw = ImguiGLFW::new(&mut imgui, &mut window);
	imgui.io_mut().display_size = [winWidth as f32, winHeight as f32];
	// let (fbx, fby) = window.get_framebuffer_size();
	// imgui.io_mut().display_framebuffer_scale = [fbx as f32 / winWidth as f32, fbx as f32 / winWidth as f32];
	imgui.io_mut().display_framebuffer_scale = [1.0, 1.0];
	println!("ImGUI initialized, version: {}", ImGui::dear_imgui_version());
	
	let mut camera = Camera {
		pos: Point3::new(0.0, 0.0, 3.0),
		..Camera::default()
	};
	
	let mut firstMouse = true;
	let mut mouseMode = false;
	let mut lastMX: f32 = winWidth as f32 / 2.0;
	let mut lastMY: f32 = winHeight as f32 / 2.0;
	
	let mut dt: f32;
	let mut lastFrameTime: f32 = 0.0;
	
	let shader = Shader::new(
		"resources/shaders/vertex.vert",
		"resources/shaders/fragment.frag",
	);
	
	let (VAO, VBO, EBO, elementCount) = unsafe {
		gl::Enable(gl::DEPTH_TEST);
		gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

		// gl::Enable(gl::CULL_FACE);
		// gl::CullFace(gl::FRONT);

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
            -0.5, 0.5, 0.5, // top left
            0.0, 0.5, 0.0, // top
            0.5, 0.5, 0.5, // top right
            0.5, 0.0, 0.0, // right
            0.5, -0.5, 0.5, // bottom right
            0.0, -0.5, 0.0, // bottom
            -0.5, -0.5, 0.5, // bottom left
            -0.5, 0.0, 0.0, // left
        ];
        let indices = [
            0, 1, 7, // top left
            1, 2, 3, // top right
            3, 4, 5, // bottom right
            7, 5, 6, // bottom left
            1, 3, 5, 1, 5, 7,
        ];
        let (mut VAO, mut VBO, mut EBO) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);
        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<GLfloat>()) as GLsizeiptr,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        (VAO, VBO, EBO, indices.len() as GLsizei)
    };
	
	let mut lineRenderer = LineRenderer::new(1024);
	
	println!("Stating main loop");
    while !window.should_close() {
        // pre-frame time logic
		let frameTime = glfw.get_time() as f32;
		dt = frameTime - lastFrameTime;
		lastFrameTime = frameTime;
		
		// events
		for (_, event) in glfw::flush_messages(&events) {
			imguiGlfw.handle_event(&mut imgui, &event);
			match event {
				glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
					(winWidth, winHeight) = (width as u32, height as u32);
					imgui.io_mut().display_size = [winWidth as f32, winHeight as f32];
					gl::Viewport(0, 0, width, height)
				},
				glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
					window.set_should_close(true)
				},
				glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
					if mouseMode {
						window.set_cursor_mode(glfw::CursorMode::Disabled);
					} else {
						window.set_cursor_mode(glfw::CursorMode::Normal);
					}
					mouseMode = !mouseMode;
				},
				glfw::WindowEvent::CursorPos(cx, cy) => {
					let (cx, cy) = (cx as f32, cy as f32);
					if firstMouse {
						lastMX = cx;
						lastMY = cy;
						firstMouse = false;
					}
					
					let xo = cx - lastMX;
					let yo = lastMY - cy;
					
					lastMX = cx;
					lastMY = cy;
					
					if mouseMode {
						camera.processMouseMovement(xo, yo, true);
						// window.set_cursor_pos(winWidth as f64 / 2.0, winHeight as f64 / 2.0);
					}
				},
				glfw::WindowEvent::Scroll(_, yo) => {
					if mouseMode {
						camera.processMouseScroll(yo as f32);
					}
				},
				_ => {},
			}
		}
		
		// update/input
		if window.get_key(Key::Space) == Action::Press {
			camera.processMovement(Movement::Up, dt);
		}
		if window.get_key(Key::LeftShift) == Action::Press {
			camera.processMovement(Movement::Down, dt);
		}
		if window.get_key(Key::W) == Action::Press {
			camera.processMovement(Movement::Forward, dt);
		}
		if window.get_key(Key::S) == Action::Press {
			camera.processMovement(Movement::Backward, dt);
		}
		if window.get_key(Key::A) == Action::Press {
			camera.processMovement(Movement::Left, dt);
		}
		if window.get_key(Key::D) == Action::Press {
			camera.processMovement(Movement::Right, dt);
		}

		let white = vec3(1.0, 1.0, 1.0);
		let red = vec3(1.0, 0.0, 0.0);
		let green = vec3(0.0, 1.0, 0.0);
		let blue = vec3(0.0, 0.0, 1.0);

		let b1 = vec3(-1.0, -1.0, -1.0);
		let b2 = vec3(1.0, -1.0, -1.0);
		let b3 = vec3(1.0, -1.0, 1.0);
		let b4 = vec3(-1.0, -1.0, 1.0);
		let t1 = vec3(-1.0, 1.0, -1.0);
		let t2 = vec3(1.0, 1.0, -1.0);
		let t3 = vec3(1.0, 1.0, 1.0);
		let t4 = vec3(-1.0, 1.0, 1.0);

		lineRenderer.pushLine(b1, white, b2, red);
		lineRenderer.pushLine(b2, red, b3, green);
		lineRenderer.pushLine(b3, green, b4, blue);
		lineRenderer.pushLine(b4, blue, b1, white);

		lineRenderer.pushLine(t1, white, t2, red);
		lineRenderer.pushLine(t2, red, t3, green);
		lineRenderer.pushLine(t3, green, t4, blue);
		lineRenderer.pushLine(t4, blue, t1, white);

		lineRenderer.pushLine(b1, white, t1, white);
		lineRenderer.pushLine(b2, red, t2, red);
		lineRenderer.pushLine(b3, green, t3, green);
		lineRenderer.pushLine(b4, blue, t4, blue);

		{
			let mut theta = 0.0f32;
			let mut lastP = vec3(0.0, -5.0, 0.0);
			let mut lastC = vec3(0.0, 0.0, 0.0);
			while theta < PI * 20.0 {
				let r = theta * 0.05; // r = b * theta (with b=1)
				let x = r * theta.cos();
				let y = r * 0.5 - 5.0;
				let z = r * theta.sin();
				let c = Vector3::from_value(theta / (PI * 20.0));

				let p = vec3(x, y, z);
				lineRenderer.pushLine(lastP, lastC, p, c);
				lastP = p;
				lastC = c;

				theta += PI * 0.05;
			}
		}

        // render
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
		}
		
		shader.bind();
		
		let third = 1.0f32 / 3.0;
		// abs(mod(x,2)-1) = triangle wave
		
		let red = ((0.25 * frameTime) % 2.0 - 1.0).abs(); //time.cos() * 0.5 + 0.5;
		let green = ((0.25 * frameTime + third) % 2.0 - 1.0).abs(); //time.sin() * 0.5 + 0.5;
		let blue = ((0.25 * frameTime - third) % 2.0 - 1.0).abs();
		shader.setUniform3f("u_color", red, green, blue);
		
		let projection: Matrix4<f32> = perspective(Deg(camera.zoom), winWidth as f32 / winHeight as f32, 0.1, 100.0);
		let view = camera.getViewMatrix();
		let model: Matrix4<f32> =  Matrix4::identity();
		let pvm = projection * view * model;
		shader.setMatrix4f("u_pvm", &pvm);
		
		unsafe {
			gl::BindVertexArray(VAO);
			gl::DrawElements(gl::TRIANGLES, elementCount, gl::UNSIGNED_INT, ptr::null());
			
			let error = gl::GetError();
			if error != gl::NO_ERROR {
				panic!("OpenGL error ({})", error);
			}
		}
		
		lineRenderer.drawFlush(&pvm);

		// imgui
        let ui = imguiGlfw.frame(&mut window, &mut imgui);
        // ui.show_demo_window(&mut true);
        ui.window("ye")
            .size([170.0, 160.0], ImGui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Hello, world!".to_string());
                let s = ui.window_size();
                ui.text(format!("{}/{}", s[0], s[1]));
				
				ui.separator();
				ui.text(format!("Mouse Pos: {}/{}", lastMX, lastMY));
				ui.text(format!("Mouse Mode (1): {}", if mouseMode { "Captured" } else { "Normal" }));
				
				ui.separator();
				if ui.button("Line Renderer Toggle") {
					lineRenderer.enabled = !lineRenderer.enabled;
				}
				if ui.button("Wireframe Toggle") {
					unsafe {
						let mut mode: GLint = gl::FILL as GLint;
						gl::GetIntegerv(gl::POLYGON_MODE, &mut mode);
						if mode == gl::FILL as GLint {
							gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
						} else {
							gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
						}
					}
				}
            });
        imguiGlfw.draw(&mut imgui, &mut window);

		// Swap & Poll
        window.swap_buffers();
        glfw.poll_events();
    }
	window.set_cursor_mode(glfw::CursorMode::Normal);

	println!("Cleaning up");
	lineRenderer.destroy();
	shader.delete();
	unsafe {
        gl::DeleteBuffers(1, &EBO);
        gl::DeleteBuffers(1, &VBO);
        gl::DeleteVertexArrays(1, &VAO);
    }
}
