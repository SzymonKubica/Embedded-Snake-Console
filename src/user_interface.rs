use crate::common::{BOARD_SIZE, MAX_SCORE};
use crate::internal_representation::game_board::BoardCell;
use crate::internal_representation::game_state::GameSpeed;

pub fn print_score(score: u8) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
    assert!(score as usize <= MAX_SCORE);

    let digits = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];
    let first_digit = digits[score as usize / 10_usize];
    let second_digit = digits[score as usize % 10_usize];

    print_lines(join_pictures(first_digit, second_digit))
}

pub fn print_speed(speed: GameSpeed) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
    let speed_number = match speed {
        GameSpeed::Slow   => ONE,
        GameSpeed::Normal => TWO,
        GameSpeed::Fast   => THREE,
    };
    print_lines(join_pictures(S, speed_number))
}

pub fn print_trophy() -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
    print_lines(TROPHY)
}

pub fn print_selection_arrows() -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
    print_lines(ARROWS)
}

pub fn print_up_down_arrows() -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
    print_lines(UP_OR_DOWN)
}

pub fn print_map(index: usize) -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {
    print_lines(MAPS[index])
}

fn print_lines(lines: [u8; BOARD_SIZE])
    -> [[BoardCell; BOARD_SIZE]; BOARD_SIZE] {

    let mut screen: [[BoardCell; BOARD_SIZE]; BOARD_SIZE] = Default::default();

    /*
     The user interface pictures are encoded using binary u8 numbers. A given
     picture is represented by an array of eight unsigned integers, and each
     of them represents a given row of the 8x8 matrix screen. In order to
     turn that way of encoding pictures into a game board, we need to iterate
     over rows and given each of those binary numbers we need to pass over it
     using a "binary mask" which is essentially a power of 2 that we shift
     leftwards. The way it works is as follows, suppose we wanto to print the
     following row: 10110111, then in order to do it we need to find the bits
     within that row which are set. That is achieved by taking the mask,
     which at first is initialised to 00000001 and performing logical AND
     of the row and the mask. If the result is non-zero, it means that the
     row had the least significant bit set and we can mark that on the board.
     To perform the next iteration, we multiply the mask by two which shifts
     it to the left (logical shift left), then we perform AND and check the
     result again. When shifting we need to check if the mask is below the
     upper bound of the u8 to avoid overflow (it can happen after the last
     iteration).
    */

    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if is_bit_set(lines[i], j) {
                screen[i][BOARD_SIZE - j - 1] = BoardCell::Obstacle;
            }
        }
    }

    screen
}

fn is_bit_set(number: u8, index: usize) -> bool {
    number & 0b_1 << index as u8 != 0
}

fn join_pictures(first: [u8; BOARD_SIZE], second: [u8; BOARD_SIZE])
-> [u8; BOARD_SIZE] {

    let mut joined: [u8; BOARD_SIZE] = Default::default();
    for i in 0..BOARD_SIZE {
        joined[i] = (first[i] << (BOARD_SIZE/2)) + second[i];
    }

    joined
}

const TROPHY: [u8; BOARD_SIZE] = [
0b_11111111, // 11111111
0b_11000011, // 11    11
0b_01100110, //  11  11
0b_00111100, //   1111
0b_00011000, //    11
0b_00011000, //    11
0b_00111100, //   1111
0b_01111110];//  111111

const ARROWS: [u8; BOARD_SIZE] = [
0b_00010000, //    1
0b_00111000, //   111
0b_00010010, //    1  1
0b_01011111, //  1 11111
0b_11111010, // 11111 1
0b_01001000, //  1  1
0b_00011100, //    111
0b_00001000];//     1
             //
const UP_OR_DOWN: [u8; BOARD_SIZE] = [
0b_00011000, //    1
0b_00111100, //   111
0b_01011010, //    1  1
0b_00011000, //  1 11111
0b_00011000, // 11111 1
0b_01011010, //  1  1
0b_00111100, //    111
0b_00011000];//     1

const ZERO: [u8; BOARD_SIZE] = [
0b_0110, //  11
0b_1001, // 1  1
0b_1001, // 1  1
0b_1001, // 1  1
0b_1001, // 1  1
0b_1001, // 1  1
0b_1001, // 1  1
0b_0110];//  11

const ONE: [u8; BOARD_SIZE] = [
0b_0001, //    1
0b_0011, //   11
0b_0101, //  1 1
0b_1001, // 1  1
0b_0001, //    1
0b_0001, //    1
0b_0001, //    1
0b_0001];//    1

const TWO: [u8; BOARD_SIZE] = [
0b_0110, //  11
0b_1001, // 1  1
0b_0001, //    1
0b_0010, //   1
0b_0100, //  1
0b_1000, // 1
0b_1000, // 1
0b_1111];// 1111

const THREE: [u8; BOARD_SIZE] = [
0b_0110, //  11
0b_1001, // 1  1
0b_1001, // 1  1
0b_0010, //   1
0b_0001, //    1
0b_1001, // 1  1
0b_1001, // 1  1
0b_0110];//  11

const FOUR: [u8; BOARD_SIZE] = [
0b_0001, //    1
0b_0011, //   11
0b_0101, //  1 1
0b_1001, // 1  1
0b_1111, // 1111
0b_0001, //    1
0b_0001, //    1
0b_0001];//    1

const FIVE: [u8; BOARD_SIZE] = [
0b_1111, // 1111
0b_1000, // 1
0b_0100, //  1
0b_0010, //   1
0b_0001, //    1
0b_1001, // 1  1
0b_1001, // 1  1
0b_0110];//  11

const SIX: [u8; BOARD_SIZE] = [
0b_0110, //  11
0b_1001, // 1  1
0b_1000, // 1
0b_1110, // 111
0b_1001, // 1  1
0b_1001, // 1  1
0b_1001, // 1  1
0b_0110];//  11

const SEVEN: [u8; BOARD_SIZE] = [
0b_1111, // 1111
0b_0001, //    1
0b_0010, //   1
0b_0100, //  1
0b_1000, // 1
0b_1000, // 1
0b_1000, // 1
0b_1000];// 1

const EIGHT: [u8; BOARD_SIZE] = [
0b_0110, //  11
0b_1001, // 1  1
0b_1001, // 1  1
0b_0110, //  11
0b_1001, // 1  1
0b_1001, // 1  1
0b_1001, // 1  1
0b_0110];//  11

const NINE: [u8; BOARD_SIZE] = [
0b_0110, //  11
0b_1001, // 1  1
0b_1001, // 1  1
0b_0111, //  111
0b_0001, //    1
0b_0001, //    1
0b_1001, // 1  1
0b_0110];//  11

const S: [u8; BOARD_SIZE] = [
0b_0110, //  11
0b_1001, // 1  1
0b_1000, // 1
0b_0100, //  1
0b_0010, //   1
0b_0001, //    1
0b_1001, // 1  1
0b_0110];//  11

pub const MAPS_NUMBER: usize = 7;

const MAPS: [[u8; BOARD_SIZE]; MAPS_NUMBER] =
[DEFAULT, SEVEN_BY_SEVEN, SIX_BY_SIX, FIVE_BY_FIVE, ZIG_ZAG, SQUEEZE, DOUGHNUT];

const DEFAULT: [u8; BOARD_SIZE] = [
0b_00000000,
0b_00000000,
0b_00000000,
0b_00000000,
0b_00000000,
0b_00000000,
0b_00000000,
0b_00000000];

const SEVEN_BY_SEVEN: [u8; BOARD_SIZE] = [
0b_00000001,
0b_00000001,
0b_00000001,
0b_00000001,
0b_00000001,
0b_00000001,
0b_00000001,
0b_11111111];

const SIX_BY_SIX: [u8; BOARD_SIZE] = [
0b_00000011,
0b_00000011,
0b_00000011,
0b_00000011,
0b_00000011,
0b_00000011,
0b_11111111,
0b_11111111];

const FIVE_BY_FIVE: [u8; BOARD_SIZE] = [
0b_00000111,
0b_00000111,
0b_00000111,
0b_00000111,
0b_00000111,
0b_11111111,
0b_11111111,
0b_11111111];

const ZIG_ZAG: [u8; BOARD_SIZE] = [
0b_00100000,
0b_00100000,
0b_00100000,
0b_00100000,
0b_00000100,
0b_00000100,
0b_00000100,
0b_00000100];

const SQUEEZE: [u8; BOARD_SIZE] = [
0b_00011000,
0b_00011000,
0b_00011000,
0b_00000000,
0b_00000000,
0b_00011000,
0b_00011000,
0b_00011000];

const DOUGHNUT: [u8; BOARD_SIZE] = [
0b_00000000,
0b_00000000,
0b_00000000,
0b_00011000,
0b_00011000,
0b_00000000,
0b_00000000,
0b_00000000];
