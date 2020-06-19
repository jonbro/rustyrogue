use wasm_bindgen::prelude::*;
use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::*;
use std::cmp::{max, min};
use nalgebra as na;
use na::{Vector3};

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}
struct State {
    time: f32,
    ecs: World
}

struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, 
                        WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos) : Self::SystemData) {
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79 , max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}
fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
fn get_height(x: f32, y: f32, z : f32) -> f32 {
    ((x*0.18).sin()*(x*0.2).sin() + (y*0.23+x*0.15).cos())*0.25+0.5
}
fn get_height_v(v : Vector3<N>) -> f32 where N: na::RealField{
    get_height(v.x, v.y, 0.0)
}
impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
fn calc_normal(x : f32, y : f32) -> Vector3 {
    const e : f32 = 0.001;
    const p : Vector3 = Vector3::new(x,y,0);
    Vector3::new(get_height_v(p + Vector3::new(e, 0.,0.)-get_height_v(p),get_height_v(p + Vector3::new(0., e,0.)-get_height_v(p),get_height_v(p + Vector3::new(0., 0.,e)-get_height_v(p)).normalize()

} 
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        self.time += 1;
        ctx.cls();
        let a = Vector3::new(1, 0, 0);
        let b = Vector3::new(-1, 0, 0);
        ctx.print(1, 2, a.dot(&b));
        player_input(self, ctx);

        self.run_systems();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for x in 0..80 {
            for y in 0..50 {
                let h = get_height(x,y);
                ctx.set(x,y, RGB::from_f32(h,h,h), RGB::from_f32(h,h,h), rltk::to_cp437(' '));
            }
        }

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }
    rltk::main_loop(context, gs)
}

// entry point for the wasm-pack module
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue>
{
    main().expect("error in main");
    Ok(())
}
