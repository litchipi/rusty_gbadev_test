#![no_std]
#![no_main]
#![feature(isa_attribute)]

use rustygba::prelude::*;

#[derive(Debug)]
pub struct Game {
    screencolor: Color,
    nframe: u8,
    nb_timer0_irq: u32,
    test_var: bool,
}

#[derive(Debug)]
pub struct GameSave {
    test_var: bool
}

impl GameState for Game {
    type SaveType = GameSave;
    fn get_gamesave(&self) -> GameSave {
        let gs = GameSave {
            test_var: self.test_var.clone()
        };
        gs
    }

    fn load_gamesave(&mut self, data: GameSave) {
        info!("Loading {:?}", data);
        self.test_var = data.test_var;
    }
}

const START_COLOR: Color = colors::color_from_hex("dd33dd");

impl Game {
    pub fn new() -> Game {
        Game {
            screencolor: START_COLOR,
            nframe: 0,
            nb_timer0_irq: 0,
            test_var: true,
        }
    }
}

fn setup() -> GbaSystem<Game> {
    let display_conf = GraphicsConfiguration::default();
    let irq_conf = IrqConfiguration::default();
    let mut sys = GbaSystem::<Game>::new(Game::new(), display_conf, irq_conf);
    sys.irq.set_timer_raw(0, 80, 1);
    sys.irq.set_timer_secs(1, 2.0);
    info!("Changes color every 2 secs");
    sys.irq.enable_selected_irq();

    sys.graphics.fill_screen(sys.game.screencolor);
    sys
}

// Saves game with test_var = true
// Set test_var = false
// Load game with test_var = false, must become test_var = true
fn gameloop(sys: &mut GbaSystem<Game>) {
    if sys.game.nframe >= 60 {
        sys.game.nframe = 0;
        info!("\n\n\n{:?}", sys.game);
    } else {
        if sys.game.nframe == 25 {
            info!("{:?}", sys.game);
        } else if sys.game.nframe == 10 {
            sys.save();
            info!("Saving ... {:?}", sys.game);
            sys.game.test_var = false;
        } else if sys.game.nframe == 40 {
            sys.load();
        }
        sys.game.nframe += 1;
    }
}

// WARNING
//  Putting messages in interruptions WILL make the game crash
fn timer0_handler(sys: &mut GbaSystem<Game>) {
    sys.game.nb_timer0_irq += 1;
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
