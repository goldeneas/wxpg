use cgmath::{Angle, Deg, Matrix4, Point3, Rad, Vector3};

use crate::modules::input_server::InputServer;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub type CameraUniform = [[f32 ; 4] ; 4];

pub struct CameraTransform {
    position: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    znear: f32,
    zfar: f32,
    yaw: f32,
    pitch: f32,
    fovy: f32,
}

impl CameraTransform {
    pub fn new(width: f32, height: f32) -> Self {
        let position = Point3::from((0.0, 0.0, 0.0));
        let target = Point3::from((0.0, 0.0, 3.0));
        let up = cgmath::Vector3::unit_y();
        let aspect = width / height;
        let znear = 0.1;
        let zfar = 100.0;
        let yaw = 0.0;
        let pitch = 0.0;
        let fovy = 45.0;

        Self {
            fovy,
            yaw,
            pitch,
            target,
            position,
            up,
            aspect,
            znear,
            zfar,
        }
    }

    pub fn uniform(&self) -> CameraUniform {
        let view = Matrix4::look_at_rh(
            self.position,
            self.target,
            self.up
        );
        
        let proj = cgmath::perspective(
            Deg(self.fovy),
            self.aspect,
            self.znear,
            self.zfar
        );

        (OPENGL_TO_WGPU_MATRIX * proj * view)
            .into()
    }
}

pub struct FpsCamera {
    speed: f32,
    transform: CameraTransform,
}

impl FpsCamera {
    pub fn new(width: f32, height: f32, speed: f32) -> Self {
        let transform = CameraTransform::new(width, height);

        Self {
            speed,
            transform,
        }
    }

    pub fn update(&mut self, input_server: &InputServer) {
        self.update_position(input_server);

        let mouse_delta = input_server.mouse_delta();
        self.update_view(mouse_delta);
    }

    fn update_view(&mut self, mouse_delta: (f64, f64)) {
        let transform = &mut self.transform;

        transform.yaw += mouse_delta.0 as f32;
        transform.pitch += mouse_delta.1 as f32;

        if transform.pitch > 89.0 {
            transform.pitch = 89.0;
        } else if transform.pitch < -89.0 {
            transform.pitch = -89.0;
        }

        let yaw_rad = Rad(transform.yaw);
        let pitch_rad = Rad(transform.pitch);
        transform.target = (
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos()
        ).into();
    }

    // todo add delta time
    // todo add relative movement with cross product
    fn update_position(&mut self, input_server: &InputServer) {
        let transform = &mut self.transform;

        if input_server.is_pressed("camera_up") {
            transform.position.y += self.speed;
        }

        if input_server.is_pressed("camera_right") {
            transform.position.x += self.speed;
        }

        if input_server.is_pressed("camera_down") {
            transform.position.y -= self.speed;
        }

        if input_server.is_pressed("camera_left") {
            transform.position.y -= self.speed;
        }

        if input_server.is_pressed("camera_front") {
            transform.position.z += self.speed;
        }

        if input_server.is_pressed("camera_back") {
            transform.position.z -= self.speed;
        }
    }

    pub fn transform(&self) -> &CameraTransform {
        &self.transform
    }
}
