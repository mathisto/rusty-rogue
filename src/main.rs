use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hullo, Singularity6!")
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("A Roguelike Resume")
        .build()?;
    let mut gs = State{ 
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
 
    gs.ecs
        .create_entity()
        .with(Position { x: 42, y: 42 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        }) 
        .build();

    for i in 0..10 {
        gs.ecs
        .create_entity()
        .with(Position {x: i * 7, y: 20})
        .with(Renderable {
            glyph: rltk::to_cp437('Σ'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
    }

    rltk::main_loop(context, gs)
}