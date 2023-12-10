use bevy::prelude::*;

use super::{LittleBrain, LittleScorerTag, Data};
use std::any::TypeId;



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScorerList {
    None, Wander, Rest
}

impl Default for ScorerList {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScorerStep {
    Init, Evaluating, CleaningUp, Finished
}

impl Default for ScorerStep {
    fn default() -> Self {
        Self::Evaluating
    }
}




#[derive(Component, Debug, Default)]
pub struct ScorerAction {
    

}


#[derive(Component, Debug, Default)]
pub struct Score {
    pub value: f32,
    pub step: ScorerStep,
    pub scorer: Option<TypeId>,
    pub previous_winner: Option<TypeId>,
    pub previous_winner2: Option<TypeId>,
    pub prevent_evaluation_finish: Option<TypeId>
}





pub fn score_management_system2(
    mut scorers: Query<(Entity, &mut Score), Without<LittleBrain>>,
) {

    }


pub fn score_management_system(
    mut cmd: Commands,
    scorers_res: Res<Data>,
    mut scorers: Query<(Entity, &mut Score), With<LittleScorerTag>>,  
){

    for (e, mut score) in scorers.iter_mut() {
        score.step = match score.step {
            ScorerStep::Init => {

                score.previous_winner2 = score.previous_winner;
                score.previous_winner = score.scorer;
                score.value = 0.;
                score.scorer = None;

                ScorerStep::Evaluating
            },
            ScorerStep::Evaluating => ScorerStep::CleaningUp,
            ScorerStep::CleaningUp => {
                cmd.entity(e).insert(LittleBrain);



                ScorerStep::Finished
            },
            ScorerStep::Finished => ScorerStep::Finished




        }
    }
}








#[derive(Component, Reflect, Default, Debug)]
pub struct WanderScorerTag;


pub fn wander_scorer_system(
    //cmd: &mut Commands,
    mut scorers: Query<(Entity, &mut Score), (With<WanderScorerTag>, With<LittleScorerTag>)>
){
    for (e, mut score) in scorers.iter_mut() {

        match score.step {
            ScorerStep::Evaluating=> {
                if score.value < 0.8 {
                    score.value = 0.8;
                    score.scorer.insert(TypeId::of::<WanderScorerTag>());
                }
                let tid = TypeId::of::<WanderScorerTag>();
                if let Some(id) = score.previous_winner {
                    if tid == id {
                        if score.value < 0.5 {
                            score.value = 0.5;
                            score.scorer.insert(tid);
                        }
                    }
                    else {
                        if score.value < 1. {
                            score.value = 1.;
                            score.scorer.insert(tid);
                        }
                    }
                    
                }
                else {
                    if score.value < 1. {
                        score.value = 1.;
                        score.scorer.insert(tid);
                    }
                }
            },
            ScorerStep::CleaningUp=> {
                // if score.scorer == ScorerList::Wander {
                //     cmd.entity(e).insert(WanderAction);
                // }
            }
            _ => println!("Not supported step")
        };
    }
}





#[derive(Component, Reflect, Default, Debug)]
pub struct RestScorerTag;





pub fn rest_scorer_system(
    //cmd: &mut Commands,
    mut scorers: Query<(Entity, &mut Score), (With<RestScorerTag>, With<LittleScorerTag>)>
){
    for (e, mut score) in scorers.iter_mut() {
        if score.step == ScorerStep::Evaluating {
                if score.value < 0.8 {
                    score.value = 0.8;
                    score.scorer.insert(TypeId::of::<RestScorerTag>());
                }
        }
        else if score.step == ScorerStep::CleaningUp {
            // if score.scorer == ScorerList::Rest {
            //     cmd.entity(e).insert(RestAction);
            // }
        }
    }
}


