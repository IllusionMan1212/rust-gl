use glfw::{Action, Key};
use glm;
pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
    _speed: f32,
    pub sensitivity: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: glm::vec3(0.0, 3.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            pitch: 0.0,
            yaw: -90.0,
            _speed: 0.0,
            speed: 10.0,
            sensitivity: 0.05,
            fov: 45.0,
        }
    }

    pub fn handle_mouse_input(&mut self, xoffset: f32, yoffset: f32, is_cursor_captured: bool) {
        if !is_cursor_captured { return }
        let xoffset = xoffset * self.sensitivity;
        let yoffset = yoffset * self.sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let front = glm::vec3(
            self.pitch.to_radians().cos() * self.yaw.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.pitch.to_radians().cos() * self.yaw.to_radians().sin(),
        );
        self.front = glm::normalize(front);
    }

    pub fn handle_mouse_scroll(&mut self, yoffset: f32, is_cursor_captured: bool) {
        if !is_cursor_captured { return }
        self.fov -= yoffset;

        if self.fov <= 1.0 {
            self.fov = 1.0;
        }
        if self.fov >= 45.0 {
            self.fov = 45.0;
        }
    }

    pub fn handle_keyboard(&mut self, window: &mut glfw::Window, is_cursor_captured: bool) {
        if !is_cursor_captured { return }
        if window.get_key(Key::W) == Action::Press {
            self.position = self.position + (self.front * self._speed);
        }
        if window.get_key(Key::S) == Action::Press {
            self.position = self.position - (self.front * self._speed);
        }
        if window.get_key(Key::A) == Action::Press {
            self.position = self.position - (glm::normalize(glm::cross(self.front, self.up)) * self._speed);
        }
        if window.get_key(Key::D) == Action::Press {
            self.position = self.position + (glm::normalize(glm::cross(self.front, self.up)) * self._speed);
        }
    }

    pub fn update_speed(&mut self, delta_time: f32) {
        self._speed = self.speed * delta_time;
    }

    pub fn reset(&mut self) {
        *self = Camera::new();
    }
}

