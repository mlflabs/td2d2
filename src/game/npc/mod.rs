use bevy::prelude::*;
use big_brain::{thinker::Thinker, pickers::FirstToScore, actions::Steps};

use super::{
        MAP_MID_Z,
        };



#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
/// Demo marker component
pub struct NpcComponent;



pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        
      
            //.register_type::<()
           
            // .add_systems(Startup,(
            //     spawn_npc,
            // ));
            // .add_plugins(DefaultPlugins.set(LogPlugin {
            //     // Use `RUST_LOG=big_brain=trace,thirst=trace cargo run --example
            //     // thirst --features=trace` to see extra tracing output.
            //     filter: "big_brain=debug,thirst=debug".to_string(),
            //     ..default()
            // }))
            //.add_plugins()
            // .add_systems(
            //     PreUpdate,
            //     (
            //         drink_action_system.in_set(BigBrainSet::Actions),
            //         thirsty_scorer_system.in_set(BigBrainSet::Scorers),
            //     ),
            // );
    }
}




