#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]


use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT};

const WIDTH: usize = BUFFER_WIDTH;
const HEIGHT: usize = BUFFER_HEIGHT - 2;
const UPDATE_FREQUENCY: usize = 3;
const JUMP_TICKS: usize = 1;
const SPAWN_TICKS: usize = 12;

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct BarrelGame {
    cells: [[Cell; WIDTH]; HEIGHT],
    status: Status,
    player: Player,
    barrels: [Barrel; 6],
    barrels_deleted: u64,
    last_key: Option<Dir>,
    countdown: usize,
    spawn_timer: usize,
    jump_timer: usize
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Dir{
    N, S, E, W
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Status{
    Running,
    Over
}

impl Dir {
    fn reverse(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E
        }
    }

    fn left(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Cell{
    Wall,
    Empty
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Position{
    col: i16,
    row: i16
}

impl Position {
    pub fn is_legal(&self) -> bool{
        0 <= self.col && self.col < WIDTH as i16 && 0 <= self.row && self.row < HEIGHT as i16
    }

    pub fn row_col(&self) -> (usize, usize){
        (self.row as usize, self.col as usize)
    }

    pub fn neighbor(&self, d: Dir) -> Position {
        match d {
            Dir::N => Position {row: self.row - 1, col: self.col},
            Dir::S => Position {row: self.row + 1, col: self.col},
            Dir::E => Position {row: self.row,     col: self.col + 1},
            Dir::W => Position {row: self.row,     col: self.col - 1}
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Player{
    pos: Position,
    jumping: bool
}

impl Player {
    fn new(pos: Position) -> Self{
        Player{
            pos,
            jumping: false
        }
    }

    pub fn icon() -> char{
        return '~'
    }

    fn collision(&self) -> Position{
        Position {row: self.pos.row - 1, col: self.pos.col}
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Barrel {
    pos: Position,
    dir: Dir,
    active: bool
}

impl Barrel {
    fn new(pos: Position) -> Self{
        Barrel {
            pos,
            active: false,
            dir: Dir::E
        }
    }

    fn is_active(&mut self) -> bool{
        self.active
    }

    fn spawn(&mut self, pos: Position){
        self.pos = pos;
        self.dir = Dir::W;
        self.active = true;
    }

    fn neighbor(&self) -> Position{
        self.pos.neighbor(self.dir)
    }
}

const START: &'static str =
    "#                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    ################################################################################
    #                                                                              #
    #            ~                                                                 #
    ################################################################################
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #
    #                                                                              #";

impl BarrelGame {
    pub fn new() -> Self {
        let mut game = BarrelGame {
            cells: [[Cell::Empty; WIDTH]; HEIGHT],
            status: Status::Running,
            player: Player::new(Position { row: 12, col: 14 }),
            barrels: [Barrel::new(Position { row: 12, col: 78 }); 6],
            barrels_deleted: 0,
            last_key: None,
            countdown: UPDATE_FREQUENCY,
            spawn_timer: SPAWN_TICKS,
            jump_timer: JUMP_TICKS
        };
        game.reset();
        game
    }

    pub fn reset(&mut self) {
        for (row, row_chars) in START.split('\n').enumerate() {
            for (col, icon) in row_chars.trim().chars().enumerate() {
                self.translate_icon(row, col, icon);
            }
        }
        self.status = Status::Running;
        self.last_key = None;
        self.player.jumping = false;
        self.barrels_deleted = 0;
    }

    fn translate_icon(&mut self, row: usize, col: usize, icon: char) {
        match icon {
            ' ' => self.cells[row][col] = Cell::Empty,
            '#' => self.cells[row][col] = Cell::Wall,
            '~' => self.player = Player::new(Position { row: row as i16, col: col as i16 }),
            _ => panic!("Unrecognized character: '{}'", icon)
        }
    }

    pub fn cell(&self, p: Position) -> Cell {
        self.cells[p.row as usize][p.col as usize]
    }

    pub fn player_at(&self, p: Position) -> bool {
        p == self.player.pos
    }

    pub fn cell_pos_iter(&self) -> RowColIter {
        RowColIter { row: 0, col: 0 }
    }

    pub fn barrel_at(&self, p: Position) -> bool {
        for b in self.barrels.iter() {
            if b.active && b.pos == p {
                return true
            }
        }
        return false
    }


    pub fn update(&mut self) {
        if self.status == Status::Running {
            self.move_player();
            self.last_key = None;
            if self.spawn_timer_done(){
                self.barrel_spawn();
            }
            self.move_barrels();
            self.collisions();
        }
    }

    fn move_player(&mut self) {
        let jump_done = self.jumping_timer_done();
        if jump_done{
            let dir = Dir::S;
            let neighbor = self.player.pos.neighbor(dir);
            if neighbor.is_legal() {
                let (row, col) = neighbor.row_col();
                if self.cells[row][col] != Cell::Wall {
                    self.player.pos = neighbor;
                }
            }
        }else if !self.player.jumping{
            if let Some(dir) = self.last_key {
                let neighbor = self.player.pos.neighbor(dir);
                if neighbor.is_legal() {
                    let (row, col) = neighbor.row_col();
                    if self.cells[row][col] != Cell::Wall {
                        self.player.pos = neighbor;
                        self.player.jumping = true;
                    }
                }
            }
        }
    }


    fn get_spawn_pos(&mut self) -> Position{
        if !self.barrel_at(Position{row: 12, col: 78}) {
            return Position { row: 12, col: 78}
        }
        return Position{row: 500, col: 500};
    }


    fn barrel_spawn(&mut self) {
        let mut active_barrels = 0;
        for b in 0..self.barrels.len() {
            if self.barrels[b].is_active() {
                active_barrels += 1;
            }
        }
        if active_barrels < self.barrels.len() - 1 {
            for b in 0..self.barrels.len() {
                if !self.barrels[b].is_active() {
                    let pos = self.get_spawn_pos();
                    self.barrels[b].spawn(pos);
                }
            }
        }
    }

    pub fn move_barrels(&mut self) {
        for b in 0..self.barrels.len() {
            if self.barrels[b].active {
                let neighbor = self.barrels[b].neighbor();
                if neighbor.is_legal() {
                    self.barrels[b].pos = neighbor;
                } else {
                    self.barrels[b].active = false;
                }
            }
        }
    }

    fn collisions(&mut self) {
        for b in 0..self.barrels.len() {
            if self.barrels[b].pos == self.player.pos && self.barrels[b].active {
                match self.status {
                    Status::Running => self.status = Status::Over,
                    Status::Over => {}
                }
            } else if self.barrels[b].pos.col <= self.player.pos.col && self.barrels[b].active {
                self.barrels[b].active = false;
                self.barrels_deleted += 1;
            }
        }
    }

    fn jumping_timer_done(&mut self) -> bool {
        if self.player.jumping {
            if self.jump_timer <= 0 {
                self.jump_timer = JUMP_TICKS;
                self.player.jumping = false;
                return true
            } else {
                self.jump_timer -= 1;
                return false
            }
        }
        return false
    }

    fn spawn_timer_done(&mut self) -> bool{
        if self.spawn_timer <= 0 {
            self.spawn_timer = SPAWN_TICKS;
            return true
        } else {
            self.spawn_timer -= 1;
            return false
        }
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

    pub fn score(&self) -> u64 {
        self.barrels_deleted
    }

    pub fn status(&self) -> Status {
        self.status
    }


    pub fn key(&mut self, key: DecodedKey) {
        match self.status {
            Status::Over => {
                match key {
                    DecodedKey::RawKey(KeyCode::S) | DecodedKey::Unicode('s') => self.reset(),
                    _ => {}
                }
            }
            _ => {
                let key = key2dir(key);
                if key.is_some() {
                    self.last_key = key;
                }
            }
        }
    }
}


    fn key2dir(key: DecodedKey) -> Option<Dir> {
        match key {
            DecodedKey::RawKey(k) => match k {
                KeyCode::ArrowUp => Some(Dir::N),
                _ => None
            }
            DecodedKey::Unicode(c) => match c {
                'w' => Some(Dir::N),
                _ => None
            }
        }
    }


    pub struct RowColIter {
        row: usize, col: usize
    }

    impl Iterator for RowColIter {
        type Item = Position;

        fn next(&mut self) -> Option<Self::Item> {
            if self.row < HEIGHT {
                let result = Some(Position {row: self.row as i16, col: self.col as i16});
                self.col += 1;
                if self.col == WIDTH {
                    self.col = 0;
                    self.row += 1;
                }
                result
            } else {
                None
            }
        }
    }
