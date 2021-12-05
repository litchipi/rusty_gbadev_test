#![no_std]
#![no_main]
#![feature(isa_attribute)]

use rustygba::prelude::*;

pub type System = GbaSystem<Game>;

#[derive(Debug)]
pub struct Game {
    screencolor: Color,
    nframe: u8,
    nb_interrupts: u32,
}

const start_color: Color = colors::color_from_hex("dd33dd");

impl Game {
    pub fn new() -> Game {
        Game {
            screencolor: start_color,
            nframe: 0,
            nb_interrupts: 0,
        }
    }
}

fn setup() -> System {
    info!("SETUP FUNCTION");
    let display_conf = GraphicsConfiguration::default();
    let irq_conf = IrqConfiguration::default();
    let mut sys = GbaSystem::new(Game::new(), display_conf, irq_conf);
    sys.irq.set_timer_raw(0, 50, 2);
    sys.irq.set_irq(Irq::HBlank);
    sys.irq.enable_selected_irq();

    sys.graphics.fill_screen(sys.game.screencolor);
    sys
}

fn gameloop(sys: &mut System) {
    if sys.game.nframe >= 60 {
        sys.game.nframe = 0;
        sys.game.screencolor = Color(sys.game.screencolor.0.rotate_left(5));
        info!("{:?}", sys.game);
        sys.graphics.fill_screen(sys.game.screencolor);
    } else {
        sys.game.nframe += 1;
    }
}

// WARNING
//  Putting messages in interruptions WILL make the game crash
fn vblank_handler(_sys: &mut System) {}
fn hblank_handler(_sys: &mut System) {}
fn vcount_handler(_sys: &mut System) {}
fn timer0_handler(sys: &mut System) {
    sys.game.nb_interrupts += 1;
}
fn timer1_handler(_sys: &mut System) {}

gba_game!(setup, gameloop, Game);
set_irq_functions!(
    vblank_handler,
    hblank_handler,
    vcount_handler,
    timer0_handler,
    timer1_handler
);
