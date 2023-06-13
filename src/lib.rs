#![no_std]

use core::{arch::wasm32, panic::PanicInfo};

const GAMEPAD1: *const u8 = 0x16 as *const u8;

const BUTTON_LEFT: u8 = 1 << 4;
const BUTTON_RIGHT: u8 = 1 << 5;
const BUTTON_UP: u8 = 1 << 6;
const BUTTON_DOWN: u8 = 1 << 7;

extern "C" {
    fn vline(x: i32, y: i32, len: u32);
}

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

#[no_mangle]
unsafe fn update() {
    if *GAMEPAD1 & BUTTON_UP != 0 {
        vline(80, 20, 120);
    }
}

const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

/// Check if the map contains a wall at a point.
fn point_in_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize) != 0),
        None => true,
    }
}

fn move_unless_into_wall(&mut state: &mut State, x: f32, y: f32) {
    if point_in_wall(x, y) {
        return;
    }
    (state.player_x, state.player_y) = (x, y);
}

struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
}

static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
};

const STEP_SIZE: f32 = 0.045;

impl State {
    /// move the character
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
        let rotate_direction =
            if left {
                1
            } else if right {
                -1
            } else {
                0
            };
        self.player_angle += (STEP_SIZE * rotate_direction);

        let move_direction =
            if up {
                1
            } else if down {
                -1
            } else {
                0
            };
        let x = self.player_x + cosf(self.player.angle) * STEP_SIZE * move_direction;
        let y = self.player_y + sinf(self.player_angle) * STEP_SIZE * move_direction;
        if point_in_wall(x, y) {
            return;
        }
        (state.player_x, state.player_y) = (x, y);
    }
}
