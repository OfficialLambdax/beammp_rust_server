use nalgebra::*;

use std::time::{Instant, Duration};

#[derive(Default, Clone, Debug)]
pub struct Car {
    pub car_json: String,

    pub pos: Vector3<f64>,
    pub rot: Quaternion<f64>,
    pub vel: Vector3<f64>,
    pub rvel: Vector3<f64>,
    pub tim: f64,
    pub ping: f64,
    pub last_pos_update: Option<Instant>,
	
	// if we ever want to make the vehicle electrics available to the lua, we have to track the states and reset the states with every vehicle reset/edit, except for some, like absMode, the lights, esc etc.. Values can be string, float, bool, int.. and modded cars can come with custom electrics
	//pub electrics: HashMap<String, ...>
	
	// could contain the id's or playernames from the players that spectate this car.
	// usefull for lua.
	//pub spectated_by: HashMap<..>
}

impl Car {
    pub fn new(car_json: String) -> Self {
        Self {
            car_json: car_json,

            ..Default::default()
        }
    }

    pub fn pos(&self) -> Vector3<f64> {
        self.pos + self.vel * self.last_pos_update.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0)
    }

    pub fn rotation(&self) -> Quaternion<f64> {
        let t = self.last_pos_update.map(|t| t.elapsed().as_secs_f64()).unwrap_or(0.0);
        self.rot + UnitQuaternion::from_euler_angles(self.rvel.x * t, self.rvel.y * t, self.rvel.z * t).quaternion()
    }
}
