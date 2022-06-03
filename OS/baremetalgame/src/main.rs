#![no_std]
#![no_main]

// To run paste into terminal: qemu-system-x86_64 -drive format=raw,file=target\x86_64-blog_os\debug\bootimage-baremetalgame.bin -L "c:\Program files\qemu"

use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::clear_screen;
use baremetalgame::MainGame;
use baremetalgame::game_main::BarrelGame;
use crossbeam::atomic::AtomicCell;
use pluggable_interrupt_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .start()
}

lazy_static! {
    static ref GAME: Mutex<MainGame> = Mutex::new(BarrelGame::new());
}

fn tick() {
    baremetalgame::tick(&mut GAME.lock())
}

fn key(key: DecodedKey) {
    let mut game = GAME.lock();
    game.key(key);
}
