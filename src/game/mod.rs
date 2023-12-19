use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_editor_pls::prelude::*;


pub mod plugin_player;
use big_brain::thinker::ThinkerBuilder;
pub use plugin_player::*;

pub mod plugin_camera;
pub use plugin_camera::*;


pub mod little_brain;
pub use little_brain::*;

pub mod npc;
pub use npc::*;

pub mod map;
pub use map::*;

pub mod cons;
pub use cons::*;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app

            //.register_type::<Thirst>()
            //.register_type::<ThinkerBuilder>()
        // .add_systems(Startup, setup_camera)
            // .add_systems(
            //     Update,
            //     ((
            //         camera_move
            //     ))
            // )
            .add_plugins((
                PlayerPlugin,
                MyCameraPlugin,
                //BrainPlugin,
                NpcPlugin,
                MapPlugin,
                LittleBrainPlugin,
                EditorPlugin::default(),
                PhysicsPlugins::default(),
                //PhysicsDebugPlugin::default()
            ));
            
    }
}
 







