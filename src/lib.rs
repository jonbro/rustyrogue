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

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0,  y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }
    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1,79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x,y,RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0.,0.,0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x,y,RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.,0.,0.), rltk::to_cp437('#'));
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
/*
git config --global user.email "you@example.com"
  git config --global user.name "Your Name"
*/
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
fn get_height(x: f32, y: f32) -> f32 {
    const PI2 : f32 = 6.28;
    //(x/50.0*PI2*0.5).sin()*50.0
    //(x/80.0*PI2*0.5).sin()*40.0
   //((x/20.0*PI2*0.5).sin()+(y/12.5*PI2*0.5).sin())*50.0
    //(((x*0.18).sin()*(x*0.2).sin() + (y*0.23+x*0.15).cos())*0.5+1.0)*40.0
    // simple bumpy terrain
    // eventually would want to interpolate values from terrain heightmap
    ((x/40.0*PI2*0.5).sin()+(y/25.0*PI2*0.5).sin())*20.0
}
fn get_height_v(v : Vector3<f32>) -> f32{
    get_height(v.x, v.y)
}
fn blocked_v(v : Vector3<f32>) -> bool {
    const cx : i32 = 20;
    const cy : i32 = 20;
    // get the height of the terrain at the center, then consider any blocks within 10 of that blocked
    let h = get_height(cx as f32, cy as f32);
    false
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}
fn calc_normal(x : f32, y : f32) -> Vector3<f32> {
    let p = Vector3::new(x,y,0.0);
    const EPS : f32 = 1.0; // or some other value
    let h = Vector3::new(EPS,0.0, 0.0);
    return Vector3::new( get_height_v(p-h.xyy()) - get_height_v(p+h.xyy()),
                            get_height_v(p-h.yxy()) - get_height_v(p+h.yxy()),
                            2.0*h.x).normalize()
} 
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        self.time += 1.0;
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
                // set color from height
                let h = get_height(x as f32, y as f32);
                // n += 1.0;
                // n *= 0.5                            ;
                // let c = RGB::from_f32(n,n,n);
                let mut n = calc_normal(x as f32,y as f32);
                // n = n.add_scalar(1.0);
                // n *= 0.5;
                // let c = RGB::from_f32(n.x,n.y,n.z);
                //let c = RGB::from_f32((self.time * 0.01).sin(), 0.0,0.0);
                // directional light is a bit hard to follow, lets try a point light, but with no falloff
                let pointLight = Vector3::new(30.0*(self.time*0.01).sin()+40.0, 20.0*(self.time*0.01).cos()+25.0, 50.0);
                // get the dot normal from this position to the light
                let p = Vector3::new(x as f32, y as f32, h);
                let mut lN = (pointLight-p);
                let mut b = Vector3::dot(&(n*1.0), &(lN.normalize()));
                b *= 1.0;
                if(b  < 0.0)
                {
                    b = 0.0;
                }
                b *= 1.0/(lN.x.powf(2.0) + lN.y.powf(2.0) + lN.z.powf( 2.0)).sqrt()*10.0;
                //let b = Vector3::dot(&n, &Vector3::new((self.time*0.02).sin(), 0.0, 1.0).normalize());
                let c = RGB::from_f32(b, b, b);
                ctx.set(x,y, c, c, rltk::to_cp437(' '));
            }
        }
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        // for (pos, render) in (&positions, &renderables).join() {
        //     ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        // }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        time: 0.0,
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());
    
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
