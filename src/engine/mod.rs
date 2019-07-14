mod keys;

use keys::Keys;

pub struct Engine {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    keys: keys::Keys,
}

impl Engine {
    pub fn init(x: f32, y: f32, z: f32) -> Engine {
        let keys = Keys::init();
        Engine {
            x,
            y,
            z,
            keys,
        }
    }

    pub fn set_key(&mut self, key_code: u32) {
        match key_code {
            87 => self.keys.w = true,
            65 => self.keys.a = true,
            83 => self.keys.s = true,
            68 => self.keys.d = true,
            81 => self.keys.q = true,
            69 => self.keys.e = true,
            _ => (),
        }
    }

    pub fn unset_key(&mut self, key_code: u32) {
        match key_code {
            87 => self.keys.w = false,
            65 => self.keys.a = false,
            83 => self.keys.s = false,
            68 => self.keys.d = false,
            81 => self.keys.q = false,
            69 => self.keys.e = false,
            _ => (),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let rate: f32 = 1.0/800.0;
        if self.keys.w {
            self.y = self.y + dt * rate;
        }
        else if self.keys.a {
            self.x = self.x - dt * rate;
        }
        else if self.keys.s {
            self.y = self.y - dt * rate;
        }
        else if self.keys.d {
            self.x = self.x + dt * rate;
        }
        else if self.keys.q {
            self.z = self.z + dt * rate;
        }
        else if self.keys.e {
            self.z = self.z - dt * rate;
        }
    }
}