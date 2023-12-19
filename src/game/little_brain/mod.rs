use bevy::{prelude::*, utils::HashMap, ecs::storage::SparseSet};

use bevy::ecs::component::SparseStorage;
use bevy_xpbd_2d::parry::utils::hashmap;
use std::any::TypeId;

pub mod actions;
pub use actions::*;

pub mod scorers;
pub use scorers::*;

use super::{Player, MAP_LOW_Z};





#[derive(Component, Reflect, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct LittleScorerTag;



pub struct LittleBrainPlugin;

impl Plugin for LittleBrainPlugin {
    fn build(&self, app: &mut App) {

       
        

        app
            .register_type::<LittleBrain>()
            .register_type::<Score>()
            .add_systems(PreUpdate,(
                score_management_system,
                action_management_system
            ))
            .add_systems(Update,(
                wander_action_system,
                rest_action_system,

                wander_scorer_system,
                rest_scorer_system,
                //test_system
            ))
            .add_systems(Startup, (
                spawn_npc,
            ));

        LittleBrainBuilder::new()
            .scorer(
                WANDER_ID,
                vec![
                    WANDER_ID
                ])
            .scorer(REST_ID, vec![
                REST_ID
            ])
            //.scorer::<RestScorerTag>(REST_ID)
            .configure(app);
        // let mut services: HashMap<String, Box<dyn MyTrait>> = HashMap::new();
        // services.insert("abhi".to_string(), Box::new(Bar));
        // services.insert("rust".to_string(), Box::new(Foo));
        //  = HashMap::from([
        //     ("abhi".to_string(), Box::new(Bar)),
        //     ("rust".to_string(), Box::new(Foo))
        // ]);

        // let mut s: HashMap<TypeId, Box<dyn Component<Storage=SparseStorage>>> = HashMap::new();

        // s.insert(TypeId::of::<Test5>(), Box::new(Test5::default()));

        // s.insert("rust".to_string(), Test2);
        // s.insert("rust2".to_string(), Test);
    }
}

//////////////////////////////////////////////////////////////////////////////////


pub fn spawn_npc(
    mut commands: Commands,
) {

    commands.spawn((
        LittleBrain::default(),
        Score::default(),
        LittleScorerTag,
        WanderScorer::default(),
        RestScorerTag,
        Name::new("GreenNpc"),
        SpriteBundle {
            transform: Transform::from_xyz(2000.0, 1000.0, MAP_LOW_Z),
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        
    ));

}




pub fn action_management_system(
    mut cmd: Commands,
    data_res: Res<LittleBrainData>,
    mut query: Query<(Entity, &mut LittleBrain, &Score), Without<LittleScorerTag>>,    
){
    for (e, mut brain, score) in query.iter_mut(){
        
        match brain.state {
            ActionState::PreInit => {
                //println!("Action State:::::::::::: {:?}", brain.state);
                //brain.reset();
                //brain.action = score.scorer;
                //let comp = data_res.as_ref().scorers[&score.scorer].;
                brain.action = data_res.scorers[&score.scorer][brain.step];
                match brain.action {
                    WANDER_ID => {
                        cmd.entity(e).insert(WanderAction);
                    },
                    REST_ID => {
                        cmd.entity(e).insert(RestAction);
                    },
                    _ => {}
                }

                brain.state = ActionState::Init;   
            },
            ActionState::InitBuffer => {
                //println!("Action State:::::::::::: {:?}", brain.state);
                brain.state = ActionState::Init;
            }
            ActionState::Init => {
                //println!("Action State:::::::::::: {:?}", brain.state);
                brain.state = ActionState::Running;
            },

            ActionState::Cleanup => {
                //println!("Action State:::::::::::: {:?}", brain.state);
                brain.state = ActionState::Finished;
                match brain.action {
                    WANDER_ID => {
                        cmd.entity(e).remove::<WanderAction>();
                    },
                    REST_ID => {
                        cmd.entity(e).remove::<RestAction>();
                    },
                    _ => {}
                }
                if brain.canceled {
                    brain.state = ActionState::Finished;
                    return
                }
                let data = data_res.as_ref();
                if data.scorers[&score.scorer].len() > brain.step + 1 {
                    brain.step += 1;
                    brain.state = ActionState::PreInit;
                }
                else {
                    brain.state = ActionState::Finished;
                    
                }
            }      


            ActionState::Finished => {
                brain.reset();
                cmd.entity(e).insert(LittleScorerTag);
                //let data = data_res.as_ref();
                // let comp = data.scorers[&score.scorer][brain.step].as_ref();
                // cmd.entity(e).remove(comp);  
            }
            _ => {}
        }
    }
}
















#[derive(Resource)]
pub struct LittleBrainData {
    //pub scorers: HashMap<u32, Vec<Box<dyn Component<Storage=SparseStorage>>>>,
    pub scorers: HashMap<u32, Vec<u32>>,
}

impl LittleBrainData {
    pub fn new(scorers: HashMap<u32, Vec<u32>>) -> LittleBrainData {
        return LittleBrainData { scorers };
    }
}






pub struct LittleBrainBuilder {
    pub scorers: HashMap<u32, Vec<u32>>,
}

impl LittleBrainBuilder {
    
    pub fn new() -> LittleBrainBuilder {
        return LittleBrainBuilder {
            scorers: HashMap::new(),
        }
    }
    //<T: Default + Component<Storage = TableStorage>>
    pub fn scorer(mut self, id: u32, actions: Vec<u32>) -> LittleBrainBuilder{
        if self.scorers.contains_key(&id) {
            error!("Duplicate scorerId {:?}", id);
        }
        else {
            self.scorers.insert(id, actions);
        }
        
        self
    }

    pub fn configure(mut self, app: &mut App){
        app.insert_resource( LittleBrainData::new(self.scorers) );
    }
}






#[derive(Component, Reflect, Default, Debug)]
//#[component(storage = "SparseSet")]
pub struct Test5;



#[derive(Component, Reflect, Default, Debug)]
pub struct Test6;




pub fn test_system(
        player_query: Query<&GlobalTransform, With<Player>>,
    ){
        //print!("T1: {}", X);
    }









// /////////////////////////////////////////////////////////////////////////////////
// #[derive(Default)]
// struct MyConfig {
//     magic: f32,
// }

// fn my_system(
//     mut cmd: Commands,
//     //my_res: Res<MyStuff>,
//     // note this isn't a valid system parameter
//     config: &MyConfig,
// ) {
//     // TODO: do stuff
// }















// ////////////////////////////////////////////////////////////////
// const X:f32 = 10.0;

// pub fn test_system(
//     player_query: Query<&GlobalTransform, With<Player>>,
// ){
//     //print!("T1: {}", X);
// }


// pub fn camera_move(
//     player_query: Query<&GlobalTransform, With<Player>>,
//     mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,    
// ){
//     //print!("T1: {}", X);
// }


// ///// tag, 
// pub struct Point {
//     pub x: f32,
//     pub y: f32,
// }

// impl Point {
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }

//     // Another associated function, taking two arguments:
//     fn new(x: f32, y: f32) -> Point {
//         Point { x: x, y: y }
//     }

//     fn print(&self) {
//         //println!("Testing Point {}", self.x);
//     }


//     pub fn test_system(
//         player_query: Query<&GlobalTransform, With<Player>>,
//     ){
//         //print!("T1: {}", X);
//     }

//     pub fn test_system2(
//         &self,
//         cmd: &mut Commands,
//     ){
//         //print!("T1: {}", self.x.clone());
//     }
// }



// // pub struct TestTest5 {
// //     pub t1: f32,
// // }

// // impl TestTest5 {
// //     pub fn wander_scorer_system(
// //         cmd: &mut Commands,
// //     ){
// //         print!("T1: {}", t1);
// //     }
// // }




// #[derive(Component, Reflect, Default, Debug)]
// pub struct Test2;



// #[derive(Component, Reflect, Default, Debug)]
// pub struct Test;


// pub struct LittleBrain {
//     pub scorers: HashMap<String, Box<dyn MyTrait>>,
//     pub actions: HashMap<String, String>,
// }

// //Implement a funtion and pass that function


// enum Actions {
//     Tst, tset, test, testset,
// }

// fn cleanup_scorers<Func, T>(){

// }



// #[derive(Component, Reflect, Default, Debug)]
// pub struct LittleBrainTag;


// #[derive(Component, Reflect, Default, Debug)]
// pub struct LittleBrainExecutingActionTag;



// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }


// #[derive(Component, Default, Debug)]
// struct Foo;

// #[derive(Component, Default, Debug)]
// struct Bar;

// pub trait MyTrait {
//     fn myfunc(&self);
//   }

// impl MyTrait for Foo{
//     fn myfunc(&self){
//     }
// }

// impl MyTrait for Bar{
//     fn myfunc(&self){
//     }
// }

