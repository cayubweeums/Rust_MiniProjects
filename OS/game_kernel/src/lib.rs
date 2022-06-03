#![feature(const_generics)]
#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]


use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, plot_str, plot_num, clear_row, ColorCode, Color};
pub mod kernel_main;

use crate::kernel_main::{CursorMover};

const GAME_HEIGHT: usize = BUFFER_HEIGHT - 2;
const HEADER_SPACE: usize = BUFFER_HEIGHT - GAME_HEIGHT;

pub type KernelMain = CursorMover;

pub fn tick(game: &mut KernelMain) {
    if game.countdown_complete() {
        game.update();
    }
}

// fn draw(game: &KernelMain) {
//     draw_board(game);
// }
//
// fn draw_board(game: &KernelMain) {
//     for p in game.cell_pos_iter() {
//         let (row, col) = p.row_col();
//         let (c, color) = get_icon_color(game, p, &game.cell(p));
//         plot(col, row + HEADER_SPACE, color);
//     }
// }

// fn get_icon_color(game: &KernelMain, p: Position, cell: &Cell) -> (char, ColorCode) {
//     let (icon, foreground) =
//         if game.player_at(p) {
//             (match game.status() {
//                 Status::Over => '*',
//                 _ => '='
//             }, Color::White)
//         } else {
//             match cell {
//                 Cell::Empty => (' ', Color::Black)
//             }
//         };
//     (icon, ColorCode::new(foreground, Color::Black))
// }
