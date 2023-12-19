use bevy::prelude::*;

use super::{Score, LittleScorerTag, LittleBrainData};









#[derive(Component, Default, Debug, Reflect)]
pub enum ActionState {
    #[default] PreInit, InitBuffer, Init, Running, Cleanup, CleanupBuffer, Canceled, Finished
}


#[derive(Component, Reflect, Default, Debug)]
pub struct LittleBrain {
    pub state: ActionState,
    pub action: u32,
    pub step: usize,
    pub canceled: bool,
}

impl LittleBrain {
    pub fn reset(&mut self) -> &LittleBrain {
        self.state = ActionState::PreInit;
        self.step = 0;
        self.canceled = false;
        self
    }

    pub fn finish(&mut self) -> &LittleBrain {
        self.state = ActionState::Cleanup;
        self
    }

    pub fn cancel(&mut self) -> &LittleBrain {
        self.state = ActionState::Cleanup;
        self.canceled = true;
        self
    }
}


#[derive(Component, Reflect, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct WanderAction;


pub fn wander_action_system(
    mut actions: Query<&mut LittleBrain, With<WanderAction>>
){
    for mut action in actions.iter_mut() {
        match action.state {
            ActionState::Init => {
                //println!("*************Wander, Initial");
            },
            ActionState::Running => {
                //println!("Running Wander, Running");
                action.finish();
            },
           
            _ => {}
        };
    }
}



#[derive(Component, Reflect, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct RestAction;





pub fn rest_action_system(
    mut actions: Query<&mut LittleBrain, With<RestAction>>
){
    for mut action in actions.iter_mut() {
        match action.state {
            ActionState::Init => {
                //println!("Rest, Initial");
            },
            ActionState::Running => {
                //println!("Resting........................");
                action.finish();
            },
            _ => {}
        };
    }
}