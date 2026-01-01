#![allow(non_snake_case)]
/* Based on https://github.com/bwasty/learn-opengl-rs/blob/master/src/camera.rs */

use cgmath;
use cgmath::prelude::*;
use cgmath::vec3;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Movement {
    Up,
    Down,
    Forward,
    Backward,
    Left,
    Right,
}
use self::Movement::*;

// Default values
const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 2.5;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
	pub pos: Point3,
	pub front: Vector3,
	pub up: Vector3,
	pub right: Vector3,
	pub worldUp: Vector3,
	
	pub yaw: f32,
	pub pitch: f32,
	
	pub speed: f32,
	pub sensitivity: f32,
	pub zoom: f32,
}

impl Default for Camera {
	fn default() -> Self {
		let mut camera = Camera {
			pos: Point3::new(0.0, 0.0, 0.0),
			front: vec3(0.0, 0.0, -1.0),
			up: Vector3::zero(),
			right: Vector3::zero(),
			worldUp: Vector3::unit_y(),
			
			yaw: YAW,
			pitch: PITCH,
			
			speed: SPEED,
			sensitivity: SENSITIVITY,
			zoom: ZOOM,
		};
		camera.updateVectors();
		camera
	}
}

#[allow(dead_code)]
impl Camera {
	pub fn getViewMatrix(&self) -> Matrix4 {
		// Matrix4::look_at(self.pos, self.pos + self.front, self.up)
		Matrix4::look_at_rh(self.pos, self.pos + self.front, self.up)
	}
	
	pub fn processMovement(&mut self, dir: Movement, dt: f32) {
		let speed = self.speed * dt;
		match dir {
			Up => {self.pos += self.worldUp * speed;}
			Down => {self.pos -= self.worldUp * speed;}
			Forward => {self.pos += self.front * speed;}
			Backward => {self.pos -= self.front * speed;}
			Left => {self.pos -= self.right * speed;}
			Right => {self.pos += self.right * speed;}
		}
	}
	
	pub fn processMouseMovement(&mut self, mut xo: f32, mut yo: f32, constrainPitch: bool) {
		xo *= self.sensitivity;
		yo *= self.sensitivity;
		
		self.yaw += xo;
		self.pitch += yo;
		
		if constrainPitch {
			self.pitch = self.pitch.clamp(-89.0, 89.0);
		}
		
		self.updateVectors();
	}
	
	pub fn processMouseScroll(&mut self, yo: f32) {
		self.zoom = (self.zoom - yo).clamp(1.0, 45.0);
	}
	
	fn updateVectors(&mut self) {
		let front = Vector3 {
			x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
			y: self.pitch.to_radians().sin(),
			z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
		};
		self.front = front;
		self.right = self.front.cross(self.worldUp).normalize();
		self.up = self.right.cross(self.front).normalize();
	}
}
