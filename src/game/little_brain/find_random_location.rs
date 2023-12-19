use bevy::prelude::*;
use rand::Rng;


use super::{LittleBrain, ActionState, Destination};




#[derive(Component, Reflect, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct FindRandomLocationAction;


//&mut MoveToDestination

pub fn random_location_action_system(
    mut actions: Query<(&mut LittleBrain, &mut Destination, &Transform), With<FindRandomLocationAction>>
){
    for (mut action, mut dest, transform) in actions.iter_mut() {
        match action.state {
            ActionState::Init => {
                //println!("*************Wander, Initial");
            },
            ActionState::Running => {
                
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(-1000.0..1000.0);
                let y = rng.gen_range(-1000.0..1000.0);
                println!("Random location offset: {:?}, {:?}", x, y);
                dest.destination = Vec2::new(
                    x + transform.translation.x, 
                    y + transform.translation.y);


                action.finish();
            },
           
            _ => {}
        };
    }
}







