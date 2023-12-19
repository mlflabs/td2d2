use bevy::prelude::*;

use crate::game::{MAP_MID_Z, MOVE_DEFAULT_SPEED, MOVE_SUCCESS_BUFFER};

use super::{LittleBrain, ActionState, MOVE_TO_DES};


#[derive(Clone, Component, Debug, Reflect)]
pub struct Destination {
    // The movement speed of the actor.
    pub destination: Vec2,
    pub success_distance: f32,
}

impl Destination {
    pub fn new(des: Vec2, success_dis: f32) -> Self {
        Destination {
            destination: des,
            success_distance: success_dis,
        }
    }

    pub fn default() -> Self {
        Destination::new(Vec2::ZERO, MOVE_SUCCESS_BUFFER)
    }
}


#[derive(Clone, Component, Debug, Reflect)]
pub struct SpeedProp{
    pub value: f32,
}

impl SpeedProp {
    pub fn new(speed: f32) -> Self {
        Self { value: speed }
    }

    pub fn default() -> Self {
        SpeedProp::new(MOVE_DEFAULT_SPEED)
    }
}





pub fn move_to_destination_action_system(
    time: Res<Time>,
    mut actions: Query<(&mut LittleBrain, &mut Transform, &Destination, &SpeedProp)>
){
    for (   mut action, 
            mut transform,
            dest,
            speed) in actions.iter_mut() {
        if action.action != MOVE_TO_DES {
            return;
        }
        
        
        match action.state {
            ActionState::Init => {
                //println!("*************Wander, Initial");
            },
            ActionState::Running => {
                let delta = dest.destination - Vec2::from((transform.translation.x, transform.translation.y));

                let distance = delta.length();

                //println!("Moving {:?} to {:?}", distance, dest);

                if distance > dest.success_distance {
                    let step_size = time.delta_seconds() * speed.value;
                    let step = delta.normalize() * step_size.min(distance);
                    transform.translation += Vec3::from((step, 0.));
                } else {
                    action.finish();
                }
            },
           
            _ => {}
        };
    }
}






