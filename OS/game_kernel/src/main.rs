#![no_std]
#![no_main]

use ghost_hunter::GhostHunterGame;
use bare_metal_tracer::TracerGame;
use pluggable_interrupt_template::LetterMover;
use baremetal_game::game_core::SpaceInvadersGame;
use chicken_invaders::Game;
use baremetal_snake::MainGame;

use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::{clear_screen, BUFFER_HEIGHT};


use game_kernel::KernelMain;
use game_kernel::kernel_main::CursorMover;


use crossbeam::atomic::AtomicCell;
use pluggable_interrupt_os::println;


// To run paste into terminal: qemu-system-x86_64 -drive format=raw,file=target\x86_64-blog_os\debug\bootimage-game_kernel.bin -L "c:\Program files\qemu"



#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .start()
}

lazy_static! {
    static ref GAME: Mutex<KernelMain> = Mutex::new(CursorMover::new());
}

fn tick() {
    game_kernel::tick(&mut GAME.lock())
}

fn key(key: DecodedKey) {
    let mut game = GAME.lock();
    game.key(key);
}
