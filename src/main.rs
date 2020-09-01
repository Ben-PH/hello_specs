use specs::{Join, Read, DispatcherBuilder, WriteStorage, ReadStorage, Component, VecStorage, System, RunNow};

use specs::{World, WorldExt, Builder};

#[derive(Default)]
struct DTime(std::time::Duration);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}


struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (Read<'a, DTime>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);
    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;
        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x = pos.x + (delta.as_millis() as f32 / 1000.) * vel.x;
            pos.y = pos.y + (delta.as_millis() as f32 / 1000.) * vel.y;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .with(Velocity { x: 1.0, y: 2.0 })
        .build();
    world.insert(DTime(std::time::Duration::from_millis(50)));

    DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build()
        .dispatch(&mut world);


    world.maintain();
}
