#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]


use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot_str, ColorCode, Color, is_drawable};
use ghost_hunter::GhostHunterGame;
use bare_metal_tracer::TracerGame;
use baremetal_game::game_core::SpaceInvadersGame;
use chicken_invaders::Game;
use baremetal_snake::MainGame;
use pluggable_interrupt_template::LetterMover;
use core::ptr::null;
use pc_keyboard::KeyCode::T;


const UPDATE_FREQUENCY: usize = 3;
const WIDTH: usize = BUFFER_WIDTH;
const HEIGHT: usize = BUFFER_HEIGHT - 2;

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct CursorMover {
    letters: [char; BUFFER_WIDTH],
    cursor_pos: Position,
    countdown: usize,
    last_key: Option<KeyCode>,
    new_move: bool,
    game_running: bool,
    game_name: GameChoice
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
#[repr(u8)]
enum GameChoice {
    GhostHunter, Tracer, Letters, SpaceInvaders, Chicken, Burnett, Hodgins
}

const GAME_CHOICES: [GameChoice; NUM_GAMES] = [GameChoice::Tracer, GameChoice::Letters, GameChoice::GhostHunter, GameChoice::Burnett, GameChoice::Chicken, GameChoice::Hodgins, GameChoice::SpaceInvaders];
const NUM_GAMES: usize = 7;

// const GAME_PROCESSES: [Process; NUM_GAMES] = [Tracer(TracerGame), Letters, GhostHunter, GameChoice::Burnett.start(), GameChoice::Chicken.start(), GameChoice::Hodgins.start(), GameChoice::SpaceInvaders.start()];

impl GameChoice {
    fn start(&self) -> Process {
        match self {
            GameChoice::GhostHunter => Process::GhostHunter(GhostHunterGame::new()),
            GameChoice::Tracer => Process::Tracer(TracerGame::new()),
            GameChoice::Letters => Process::Letters(LetterMover::new()),
            GameChoice::SpaceInvaders => Process::SpaceInvaders(SpaceInvadersGame::new()),
            GameChoice::Chicken => Process::Chicken(Game::new()),
            GameChoice::Burnett => Process::Burnett(nom_noms::LetterMover::new()),
            GameChoice::Hodgins => Process::Hodgins(MainGame::new())
        }
    }
    fn name(&self) -> &'static str {
        match self {
            GameChoice::GhostHunter => "Ghost Hunter",
            GameChoice::Tracer => "Tracer",
            GameChoice::Letters => "Letter Mover",
            GameChoice::SpaceInvaders => "Space Invaders",
            GameChoice::Chicken => "Chicken Invaders",
            GameChoice::Burnett => "Daniel's Game",
            GameChoice::Hodgins => "Snake"
        }
    }
}
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Process {
    GhostHunter(ghost_hunter::MainGame),
    Tracer(TracerGame),
    Letters(LetterMover),
    SpaceInvaders(SpaceInvadersGame),
    Chicken(Game),
    Burnett(nom_noms::LetterMover),
    Hodgins(baremetal_snake::MainGame)
}
impl Process {
    fn tick(&mut self) {
        match self {
            Process::GhostHunter(game) => ghost_hunter::tick(game),
            Process::Tracer(game) => game.tick(),
            Process::Letters(game) => game.tick(),
            Process::SpaceInvaders(game) => baremetal_game::tick(game),
            Process::Chicken(game) => game.tick(),
            Process::Burnett(game) => game.tick(),
            Process::Hodgins(game) => baremetal_snake::tick(game)
        }
    }
    fn key(&mut self, key: DecodedKey) {
        match self {
            Process::GhostHunter(game) => game.key(key),
            Process::Tracer(game) => game.key(key),
            Process::Letters(game) => game.key(key),
            Process::SpaceInvaders(game) => game.key(key),
            Process::Chicken(game) => game.key(key),
            Process::Burnett(game) => game.key(key),
            Process::Hodgins(game) => game.key(key)
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Position{
    col: i16,
    row: i16
}

impl Position {
    pub fn row_col(&self) -> (usize, usize){
        (self.row as usize, self.col as usize)
    }
}

impl CursorMover {
    pub fn new() -> Self {
        let mut game = CursorMover {
            letters: ['A'; BUFFER_WIDTH],
            cursor_pos: Position{col: 0,row: 3},
            countdown: UPDATE_FREQUENCY,
            last_key: None,
            new_move: false,
            game_running: false,
            game_name: GAME_CHOICES[0]
        };
        game.start();
        game
    }

    pub fn update(&mut self) {
        // if self.game_running{
        //     let mut i = 0;
        //     for game in GAME_CHOICES.iter(){
        //         if game.name() == self.game_name.name(){
        //             GAME_PROCESSES[i].tick();
        //         }else {
        //             i += 1
        //         }
        //     }
        // }
        self.move_cursor();
        self.last_key = None;
    }

    fn start(&mut self){
        plot_str("Press S to start game", 0, 0, ColorCode::new(Color::White,Color::Black));
        plot_str("Press R to resume game",0,1,ColorCode::new(Color::White,Color::Black));
        plot_str("Press K to resume game",0,2,ColorCode::new(Color::White,Color::Black));
        let mut i = self.cursor_pos.row as usize - 3;
        let game = GAME_CHOICES[i].name();
        self.draw_games();
        plot_str(game,self.cursor_pos.col as usize, self.cursor_pos.row as usize, ColorCode::new(Color::Black, Color::White));
    }

    pub fn countdown_complete(&mut self) -> bool {
        if self.countdown == 0 {
            self.countdown = UPDATE_FREQUENCY;
            true
        } else {
            self.countdown -= 1;
            false
        }
    }

    fn move_cursor(&mut self){
        if self.new_move{
            let mut i = self.cursor_pos.row as usize - 3;
            let game = GAME_CHOICES[i].name();
            self.draw_games();
            plot_str(game,self.cursor_pos.col as usize, self.cursor_pos.row as usize, ColorCode::new(Color::Black, Color::White));
            self.new_move = false;
        }
    }

    fn draw_games(&self){
        let mut i:usize = 3;
        for game in GAME_CHOICES.iter(){
            plot_str(game.name(),0,i,ColorCode::new(Color::White,Color::Black));
            i += 1;
        }
        // TODO may need to fix how we get the name for processes
        // let mut p:usize = 3;
        // for process in self.running_games.iter(){
        //     plot_str(process.name(),20,i,ColorCode::new(Color::White,Color::Black));
        //     p += 1;
        // }
    }

    pub fn key(&mut self, key: DecodedKey) {
        // if self.game_running{
        //     match key {
        //         DecodedKey::RawKey(k) => match k {
        //             KeyCode::Escape => {
        //                 self.game_running = false;
        //                 self.start()
        //             },
        //             _ => {}
        //         },
        //         _ => {
        //             self.key2game(key)
        //         }
        //     }
        // }else {
            match key {
                DecodedKey::RawKey(code) => self.key2dir_code(code),
                DecodedKey::Unicode(char) => self.key2dir_char(char)
            }
        }
    // }

    // fn key2game(&mut self, key: DecodedKey){
    //     let mut i = 0;
    //     for game in GAME_CHOICES.iter(){
    //         if game.name() == self.game_name.name(){
    //             GAME_PROCESSES[i].key(key)
    //         }else {
    //             i += 1
    //         }
    //     }
    // }

    fn key2dir_code(&mut self, key: KeyCode){
        match key {
            KeyCode::ArrowRight => {
                self.cursor_pos.col = 0;
            }
            KeyCode::ArrowLeft => {
                self.cursor_pos.col = 0;
            }
            KeyCode::ArrowUp => {
                if self.cursor_pos.row - 1 <= 2{
                    self.cursor_pos.row = 3;
                }else{
                    self.cursor_pos.row -= 1;
                    self.new_move = true
                }
            }
            KeyCode::ArrowDown => {
                if self.cursor_pos.row + 1 >= 10{
                    self.cursor_pos.row = 9;
                }else {
                    self.cursor_pos.row += 1;
                    self.new_move = true
                }
            }
            _ => {}
        }
    }

    fn key2dir_char(&mut self, key: char){
        match key {
            's' => {
                self.start_game();
                self.game_running = true;
            }
            _ => {}
        }
    }


    fn start_game(&mut self){
        let mut i = self.cursor_pos.row as usize - 3;
        let game = GAME_CHOICES[i];
        self.game_name = game;
        game.start();
        // GAME_PROCESSES[i] = game.start();
    }

    fn run_game(&mut self){

    }

}

