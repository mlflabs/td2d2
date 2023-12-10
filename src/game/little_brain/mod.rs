use bevy::{prelude::*, utils::HashMap, ecs::storage::SparseSet};

use bevy::ecs::component::TableStorage;
use std::any::TypeId;

pub mod actions;
pub use actions::*;

pub mod scorers;
pub use scorers::*;

use super::{Player, MAP_LOW_Z};


#[derive(Component, Reflect, Default, Debug)]
pub struct LittleBrain;


#[derive(Component, Reflect, Default, Debug)]
#[component(storage = "SparseSet")]
pub struct LittleScorerTag;



pub struct LittleBrainPlugin;

impl Plugin for LittleBrainPlugin {
    fn build(&self, app: &mut App) {

       
        

        app.add_systems(PreUpdate,(
            score_management_system,
        ));
        app.add_systems(Update,(
            wander_scorer_system,
            rest_scorer_system,
            //test_system
        ));

        LittleBrainBuilder::new()
            .scorer::<Test5>()
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
        LittleBrain,
        WanderScorerTag,
        RestScorerTag,
        Name::new("WaterSource"),
        SpriteBundle {
            transform: Transform::from_xyz(4000.0, 2000.0, MAP_LOW_Z),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        
    ));

}


#[derive(Resource)]
pub struct Data {
    pub scorers: HashMap<TypeId, Box<dyn Component<Storage=TableStorage>>>,
}

impl Data {
    pub fn new(scorers: HashMap<TypeId, Box<dyn Component<Storage=TableStorage>>>) -> Data {
        return Data { scorers };
    }
}






pub struct LittleBrainBuilder {
    pub scorers: HashMap<TypeId, Box<dyn Component<Storage=TableStorage>>>,
}

impl LittleBrainBuilder {
    
    pub fn new() -> LittleBrainBuilder {
        return LittleBrainBuilder {
            scorers: HashMap::new(),
        }
    }

    pub fn scorer<T: Default + Component<Storage = TableStorage>>(mut self) -> LittleBrainBuilder{
        self.scorers.insert(TypeId::of::<T>(), Box::new(T::default()));
        self
    }

    pub fn configure(mut self, app: &mut App){
        app.insert_resource( Data::new(self.scorers) );
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

