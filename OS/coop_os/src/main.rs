#![no_std]
#![no_main]

use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::clear_screen;
use coop_os_starter::Kernel;
use crossbeam::atomic::AtomicCell;
use coop_os::Kernel;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .startup(startup)
        .cpu_loop(cpu_loop)
        .start()
}

lazy_static! {
    static ref LAST_KEY: AtomicCell<Option<DecodedKey>> = AtomicCell::new(None);
    static ref TICKS: AtomicCell<usize> = AtomicCell::new(0);
}

fn cpu_loop() -> ! {
    let mut kernel = Kernel::new();
    let mut last_tick = 0;
    kernel.draw();
    loop {
        if let Some(key) = LAST_KEY.load() {
            LAST_KEY.store(None);
            kernel.key(key);
        }
        let current_tick = TICKS.load();
        if current_tick > last_tick {
            last_tick = current_tick;
            kernel.draw_proc_status();
        }
        kernel.run_one_instruction();
    }
}

fn tick() {
    TICKS.fetch_add(1);
}

fn key(key: DecodedKey) {
    LAST_KEY.swap(Some(key));
}

fn startup() {
    clear_screen();
}