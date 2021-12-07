#![no_std]

#![no_main]
#![feature(isa_attribute)]

use rustygba::prelude::*;

#[derive(Debug)]
pub struct Game {
    screencolor: Color,
    nframe: u8,
    nb_interrupts: u32,
}

const START_COLOR: Color = colors::color_from_hex("dd33dd");

impl Game {
    pub fn new() -> Game {
        Game {
            screencolor: START_COLOR,
            nframe: 0,
            nb_interrupts: 0,
        }
    }
}

fn setup() -> GbaSystem<Game> {
    let display_conf = GraphicsConfiguration::default();
    let irq_conf = IrqConfiguration::default();
    let mut sys = GbaSystem::<Game>::new(Game::new(), display_conf, irq_conf);
    sys.irq.set_timer_raw(0, 80, 1);
    sys.irq.set_timer_secs(1, 2.5);
    sys.irq.enable_selected_irq();

    sys.graphics.fill_screen(sys.game.screencolor);
    sys
}

fn gameloop(sys: &mut GbaSystem<Game>) {
    if sys.game.nframe >= 60 {
        sys.game.nframe = 0;
        info!("{:?}", sys.game);
    } else {
        sys.game.nframe += 1;
    }
}

// WARNING
//  Putting messages in interruptions WILL make the game crash
fn timer0_handler(sys: &mut GbaSystem<Game>) {
    sys.game.nb_interrupts += 1;
}

fn vblank_handler(sys: &mut GbaSystem<Game>) {
    sys.graphics.fill_screen(sys.game.screencolor);
}
fn hblank_handler(_sys: &mut GbaSystem<Game>) {}
fn vcount_handler(_sys: &mut GbaSystem<Game>) {}
fn timer1_handler(sys: &mut GbaSystem<Game>) {
    sys.game.screencolor = Color(sys.game.screencolor.0.rotate_left(5));
}

gba_game!(setup, gameloop, Game);
set_irq_functions!(
    vblank_handler,
    hblank_handler,
    vcount_handler,
    timer0_handler,
    timer1_handler
);
