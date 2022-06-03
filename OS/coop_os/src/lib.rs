#![feature(fmt_as_str)]
#![cfg_attr(not(test), no_std)]

use iter_state_machine::examples::{Average, Pi};
use iter_state_machine::{Instruction, StateMachine};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_str, peek};
use pc_keyboard::{DecodedKey, KeyCode};
use lazy_static::lazy_static;
use core::fmt::Write;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Choice {
    Average, Pi
}

impl Choice {
    fn start(&self) -> Process {
        match self {
            Choice::Average => Process::Average(Average::new()),
            Choice::Pi => Process::Pi(Pi::new())
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Choice::Average => "Average",
            Choice::Pi => "Pi"
        }
    }
}

const NUM_PROGRAMS: usize = 2;
static PROGRAMS: [Choice; NUM_PROGRAMS] = [Choice::Average, Choice::Pi];

#[derive(Copy,Clone)]
enum Process {
    Average(Average),
    Pi(Pi)
}

impl StateMachine for Process {
    fn receive_input(&mut self, value: &str) -> Option<&'static str> {
        match self {
            Process::Average(avg) => avg.receive_input(value),
            Process::Pi(pi) => pi.receive_input(value)
        }
    }

    fn update(&mut self) {
        match self {
            Process::Average(avg) => avg.update(),
            Process::Pi(pi) => pi.update()
        }
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        match self {
            Process::Average(avg) => avg.next_instruction(),
            Process::Pi(pi) => pi.next_instruction()
        }
    }
}

lazy_static! {
    static ref TEXT_COLOR: ColorCode = ColorCode::new(Color::White, Color::Black);
    static ref REVERSE_COLOR: ColorCode = ColorCode::new(Color::Black, Color::White);
}

const MAX_PROCESSES: usize = 4;

pub struct Kernel {
    processes: [ProcessData; MAX_PROCESSES],
    current_window_pid: usize,
    scheduler: Policy,
    status_buffer: BufferRenderer
}

impl Kernel {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn draw(&mut self) {
        unimplemented!()
    }

    pub fn run_one_instruction(&mut self) {
        unimplemented!()
    }

    pub fn draw_proc_status(&mut self) {
        unimplemented!()
    }

    pub fn key(&mut self, key: DecodedKey) {
        unimplemented!()
    }
}

struct ProcessData {
    // TODO: Your code here
}

pub enum Policy {
    CurrentOnly
    // TODO: Add some more options, then handle them in pick_pid()
}

impl Policy {
    fn pick_pid(&self, current_pid: usize, processes: &[ProcessData; MAX_PROCESSES]) -> Option<usize> {
        match self {
            Policy::CurrentOnly => Some(current_pid),
            _ => None
        }
    }
}

#[derive(Copy,Clone)]
struct BufferRenderer {
    x_start: usize, y_start: usize, x_end: usize, y_end: usize,
    x_cursor: usize, keystrokes: [u8; BUFFER_WIDTH], num_keystrokes: usize
}

impl BufferRenderer {
    fn new(x_start: usize, y_start: usize, x_end: usize, y_end: usize) -> Self {
        BufferRenderer {x_start, y_start, x_end, y_end, x_cursor: x_start,
            keystrokes: [0; BUFFER_WIDTH], num_keystrokes: 0}
    }

    fn reset_cursor(&mut self) {
        self.x_cursor = self.x_start;
    }

    fn print(&mut self, s: &str) {
        for c in s.chars() {
            self.print_char(c);
        }
    }

    fn println(&mut self, s: &str) {
        self.print(s);
        self.print_char('\n');
    }

    fn print_char(&mut self, c: char) {
        if c == '\n' {
            self.scroll_vertical();
        } else {
            // TODO: Your code here
            unimplemented!()
        }
    }

    fn scroll_vertical(&mut self) {
        // TODO: Your code here
        unimplemented!()
    }

    fn clear_window(&mut self) {
        for x in self.x_start..=self.x_end {
            for y in self.y_start..=self.y_end {
                plot(' ', x, y, *TEXT_COLOR);
            }
        }
        self.reset_cursor();
    }

    fn give_process_input(&mut self, p: &mut Process) {
        let s = core::str::from_utf8(&self.keystrokes[0..self.num_keystrokes]).unwrap();
        p.receive_input(s.trim()).map(|s| self.println(s));
        self.num_keystrokes = 0;
    }

    fn receive_char(&mut self, chr: u8) {
        if self.num_keystrokes + 1 < self.keystrokes.len() {
            self.print_char(char::from(chr));
            if chr != b'\n' {
                self.keystrokes[self.num_keystrokes] = chr;
                self.num_keystrokes += 1;
            }
        }else {
            self.print_char('\n');
        }
    }
}

// let b = BufferRenderer();
// format!(b, "this is an int: {}", i);
impl core::fmt::Write for BufferRenderer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}