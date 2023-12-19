use bevy::prelude::*;

use super::{LittleBrain, LittleScorerTag};
use std::{any::TypeId, default};



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScorerList {
    None, Wander, Rest
}

impl Default for ScorerList {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Reflect, PartialEq, Eq)]
pub enum ScorerState {
    Init, 
    Evaluating, 
    CleaningUp, Finished
}

impl Default for ScorerState {
    fn default() -> Self {
        Self::Init
    }
}




#[derive(Component, Debug, Default, Reflect)]
pub struct Score {
    pub value: f32,
    pub state: ScorerState,
    pub scorer:  u32,//Option<TypeId>,
    pub previous_winner: u32,
    pub previous_winner2: u32,
    pub prevent_evaluation_finish: u32,
}




pub fn score_management_system(
    mut cmd: Commands,
    //scorers_res: Res<Data>,
    mut scorers: Query<(Entity, &mut Score), With<LittleScorerTag>>,  
){

    for (e, mut score) in scorers.iter_mut() {
        match score.state {
            ScorerState::Init => {
                //println!("Scorer Initialization");
                score.previous_winner2 = score.previous_winner;
                score.previous_winner = score.scorer;
                score.value = 0.;
                score.scorer = 0;
                score.state = ScorerState::Evaluating;
            },
            ScorerState::Evaluating => { 
                score.state = ScorerState::CleaningUp;
            },
            ScorerState::CleaningUp => {
                //println!("New Scorer state: {:?}", score);
                cmd.entity(e).remove::<LittleScorerTag>();
                score.state = ScorerState::Finished;
            },
            ScorerState::Finished => {
                score.state = ScorerState::Init;
            }
            _ => {}
        };

        //println!("Scorer Management: {:?} :: {:?}",score.state,  score);
    }
}







pub const WANDER_ID: u32 = 100;
#[derive(Component, Reflect, Default, Debug)]
pub struct WanderScorer(f32);


pub fn wander_scorer_system(
    //cmd: &mut Commands,
    mut scorers: Query<(&mut WanderScorer, &mut Score), With<LittleScorerTag>>
){
    for (mut wander, mut score) in scorers.iter_mut() {
        //println!("WanderScore {:?}", score.state);
        match score.state {
            ScorerState::Evaluating=> {
                //println!("Evaluating WanderScore");
                let mut s = 1.;
                if score.previous_winner == WANDER_ID {
                    s = 0.5;
                }
                //wander = WanderScorer(s);
                if score.value < s {
                    score.value = s;
                    score.scorer = WANDER_ID;
                }
            },
            ScorerState::CleaningUp=> {
                // if score.scorer == ScorerList::Wander {
                //     cmd.entity(e).insert(WanderAction);
                // }
            }
            _ => {}
        };
    }
}




pub const REST_ID: u32 = 110;
#[derive(Component, Reflect, Default, Debug)]
pub struct RestScorerTag;





pub fn rest_scorer_system(
    //cmd: &mut Commands,
    mut scorers: Query<(Entity, &mut Score), (With<RestScorerTag>, With<LittleScorerTag>)>
){
    for (e, mut score) in scorers.iter_mut() {
        if score.state == ScorerState::Evaluating {
                if score.value < 0.8 {
                    score.value = 0.8;
                    score.scorer = REST_ID;
                }
        }
    }
}


