#![no_std]
#![no_main]
#![feature(isa_attribute)]

use rustygba::prelude::*;

pub type System = GbaSystem<Game>;

pub struct Game {
    screencolor: Color,
    nframe: u8,
}

impl Game {
    pub fn new() -> Game {
        Game { screencolor: colors::RED, nframe: 0 }
    }
}

fn setup() -> System {
    let display_conf = GraphicsConfiguration::default();
    let irq_conf = IrqConfiguration::default();
    GbaSystem::new(Game::new(), display_conf, irq_conf)
}

fn gameloop(sys: &mut System) {
    info!("Loop {}", sys.game.nframe);
    if sys.game.nframe >= 60 {
        sys.game.nframe = 0;
        sys.game.screencolor = Color(sys.game.screencolor.0.rotate_left(5));
    } else {
        sys.game.nframe += 1;
    }
    sys.graphics.fill_screen(sys.game.screencolor);
}

fn vblank_handler(sys: &mut System) {}
fn hblank_handler(sys: &mut System) {}
fn vcount_handler(sys: &mut System) {}
fn timer0_handler(sys: &mut System) {}
fn timer1_handler(sys: &mut System) {}

set_irq_functions!(vblank_handler, hblank_handler, vcount_handler, timer0_handler, timer1_handler);
gba_game!(setup, gameloop, Game);
