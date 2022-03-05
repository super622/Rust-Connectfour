use ggez::ContextBuilder; 
use ggez::conf::{Conf, WindowMode};
use ggez::filesystem;
use ggez::event;
use ggez::mint::Point2;

use std::{env, path};
use structopt::StructOpt;

use connectfour::entities::enums::Colour;
use connectfour::entities::world_state::WorldState;

#[derive(StructOpt, Debug)]
#[structopt(name = "connectfour")]
struct Opt {
    #[structopt(short, long)]
    red: bool,

    #[structopt(short, long, default_value="0")]
    x_offset: f32,

    #[structopt(short, long, default_value="0")]
    y_offset: f32,

    #[structopt(short, long, default_value="Player1")]
    first_name: String,

    #[structopt(short, long, default_value="Player2")]
    second_name: String,
}

fn main() {
    let opt = Opt::from_args();

    let conf = Conf::new().
    window_mode(WindowMode {
        width: 736.0 + opt.x_offset,
        height: 688.0 + opt.y_offset,
        ..Default::default()
    });
    let (mut ctx, event_loop) = ContextBuilder::new("connectfour", "Pesho").
    default_conf(conf.clone()).
    build().
    unwrap();
    
    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = WorldState::new(
        &mut ctx, 
        Point2{x:opt.x_offset, y:opt.y_offset},
        if opt.red {Colour::Red} else {Colour::Green},
        opt.first_name,
        opt.second_name,
    ).unwrap();
    event::run(ctx, event_loop, state);
}
