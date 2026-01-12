#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
extern crate console_error_panic_hook;
mod game;

use crate::game::{Game, State};
use std::panic;
use std::sync::Mutex;
use wasm_bindgen::__rt::LazyLock;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/static/helper.js")]
extern "C" {
    fn print(str: &str);
    fn getWindowWidth() -> i16;
    fn getWindowHeight() -> i16;
    fn rand() -> f64;
    fn drawRect(x: i16, y: i16, w: i16, h: i16, r: u8, g: u8, b: u8, t: i16);
    fn fillRect(x: i16, y: i16, w: i16, h: i16, r: u8, g: u8, b: u8);
    fn fill3DRect(x: i16, y: i16, w: i16, h: i16, r: u8, g: u8, b: u8, t: i16, raised: bool);
    fn drawCross(x: i16, y: i16, s: i16, r: u8, g: u8, b: u8, t: i16);
    fn drawCircle(x: i16, y: i16, s: i16, r: u8, g: u8, b: u8, t: i16);
    fn sendData(str: &str, x: i16, y: i16);
    fn setStatus(str: &str);
    fn setTitle(waiting: bool);
    fn playSFX(file: &str);
}

static BOX_SIZE: i16 = 45;
static BOX_BORDER: i16 = 1;
static GRID_SIZE: i16 = 15;

static OppGameStart: LazyLock<Mutex<i8>> = LazyLock::new(|| Mutex::new(-1));
static PlayerGameStart: LazyLock<Mutex<i8>> = LazyLock::new(|| Mutex::new(-1));
static OffsetX: LazyLock<Mutex<i16>> = LazyLock::new(|| Mutex::new(0));
static OffsetY: LazyLock<Mutex<i16>> = LazyLock::new(|| Mutex::new(0));
static Player: LazyLock<Mutex<i8>> = LazyLock::new(|| Mutex::new(-1));

#[wasm_bindgen]
pub fn setHook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

static MAIN_GAME: LazyLock<Mutex<Game>> = LazyLock::new(|| Mutex::new(Game::new()));

fn resetState() {
    *PlayerGameStart.lock().unwrap() = -1;
    *OppGameStart.lock().unwrap() = -1;
    *OffsetX.lock().unwrap() = 0;
    *OffsetY.lock().unwrap() = 0;

    if (*Player.lock().unwrap() == 1) {
        setStatus("New Game Started, Your turn to Place");
        setTitle(true);
    } else {
        setStatus("New Game Started, Opponent's turn to Place");
        setTitle(false);
    }

    MAIN_GAME.lock().unwrap().resetState();
}

#[wasm_bindgen]
pub fn render() {
    let width = getWindowWidth();
    let height = getWindowHeight();
    fillRect(0, 0, width, height, 32, 32, 48);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let x = i - 7 + *OffsetX.lock().unwrap();
            let y = 7 - j + *OffsetY.lock().unwrap();
            match MAIN_GAME.lock().unwrap().getState(x, y) {
                State::Inactive => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        96,
                        96,
                        96,
                        BOX_BORDER,
                        true,
                    );
                }
                State::Activatable => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        128,
                        192,
                        64,
                        BOX_BORDER,
                        false,
                    );
                }
                State::Active => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        0,
                        160,
                        224,
                        BOX_BORDER,
                        false,
                    );
                }
                State::Cross => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        0,
                        160,
                        224,
                        BOX_BORDER,
                        false,
                    );
                    drawCross(
                        BOX_SIZE + i * BOX_SIZE + BOX_SIZE / 2 + 1,
                        BOX_SIZE + j * BOX_SIZE + BOX_SIZE / 2 + 1,
                        (BOX_SIZE - BOX_BORDER * 16) / 2,
                        255,
                        128,
                        32,
                        BOX_BORDER * 4,
                    );
                }
                State::Circle => {
                    fill3DRect(
                        BOX_SIZE + i * BOX_SIZE,
                        BOX_SIZE + j * BOX_SIZE,
                        BOX_SIZE,
                        BOX_SIZE,
                        0,
                        160,
                        224,
                        BOX_BORDER,
                        false,
                    );
                    drawCircle(
                        BOX_SIZE + i * BOX_SIZE + BOX_SIZE / 2 + 1,
                        BOX_SIZE + j * BOX_SIZE + BOX_SIZE / 2 + 1,
                        (BOX_SIZE - BOX_BORDER * 16) / 2,
                        255,
                        128,
                        32,
                        BOX_BORDER * 4,
                    );
                }
            }
        }
    }
}

fn reset() {
    playSFX("reset.mp3");
    resetState();
    render();
}

#[wasm_bindgen]
pub fn handleKeyDown(key: &str) {
    match key {
        "ArrowUp" => {
            *OffsetY.lock().unwrap() += 1;
        }
        "ArrowRight" => {
            *OffsetX.lock().unwrap() += 1;
        }
        "ArrowDown" => {
            *OffsetY.lock().unwrap() -= 1;
        }
        "ArrowLeft" => {
            *OffsetX.lock().unwrap() -= 1;
        }
        " " => {
            *OffsetX.lock().unwrap() = 0;
            *OffsetY.lock().unwrap() = 0;
        }
        "Shift" => {
            if (MAIN_GAME.lock().unwrap().Move == -1) {
                sendData("Start", 0, 0);
                if (*OppGameStart.lock().unwrap() == 0) {
                    *PlayerGameStart.lock().unwrap() = 1;
                    setStatus("Waiting for Opponent to Start New Game");
                    setTitle(false);
                    playSFX("click.mp3");
                } else if (*OppGameStart.lock().unwrap() == 1) {
                    MAIN_GAME.lock().unwrap().Move = 0;
                    reset();
                }
            }
        }
        _ => {}
    }
    render();
}

#[wasm_bindgen]
pub fn handleMouseClick(mouseX: i16, mouseY: i16) {
    let gridX = (mouseX - BOX_SIZE) / BOX_SIZE;
    let gridY = (mouseY - BOX_SIZE) / BOX_SIZE;
    if (gridX >= 0 && gridY >= 0 && gridX < GRID_SIZE && gridY < GRID_SIZE) {
        let x = gridX - 7 + *OffsetX.lock().unwrap();
        let y = 7 - gridY + *OffsetY.lock().unwrap();
        let currPlayer = *Player.lock().unwrap();

        let validClick = MAIN_GAME.lock().unwrap().doPlayerClick(x, y, currPlayer);
        print(format!("Move:{},{}", x, y).as_str());

        if (validClick) {
            let currMove = MAIN_GAME.lock().unwrap().Move;
            match currMove {
                1 | 3 => {
                    setStatus("Your turn to expand");
                    setTitle(true);
                }
                0 | 2 => {
                    setStatus("Opponent's turn to place");
                    setTitle(false);
                }
                _ => {}
            }

            render();
            sendData("Move", x, y);

            let win = MAIN_GAME.lock().unwrap().checkWin(x, y);
            if (win == currPlayer) {
                sendData("Win", x, y);
                setStatus("Your Won, Press Shift to Start a New Game");
                setTitle(true);
                playSFX("finish.mp3");
                *Player.lock().unwrap() = 3 - currPlayer;
                MAIN_GAME.lock().unwrap().Move = -1;
                *OppGameStart.lock().unwrap() = 0;
                *PlayerGameStart.lock().unwrap() = 0;
            }else{
                playSFX("click.mp3");
            }
        }
    }
}

#[wasm_bindgen]
pub fn handleResign() {
    let currPlayer = *Player.lock().unwrap();
    sendData("Resign", 0, 0);
    setStatus("You Resigned, Press Shift to Start a New Game");
    setTitle(true);
    playSFX("finish.mp3");
    *Player.lock().unwrap() = 3 - currPlayer;
    MAIN_GAME.lock().unwrap().Move = -1;
    *OppGameStart.lock().unwrap() = 0;
    *PlayerGameStart.lock().unwrap() = 0;
}

#[wasm_bindgen]
pub fn handleDataIn(str: &str, x: i16, y: i16) {
    if (str == "Join") {
        if (MAIN_GAME.lock().unwrap().Move == -1) {
            if (x == 1 || x == 2) {
                *Player.lock().unwrap() = (3 - x) as i8;
                sendData("Join", (3 - x), 0);
                reset();
            }
        }
    } else if (str == "Move") {
        let currPlayer = *Player.lock().unwrap();
        let validClick = MAIN_GAME
            .lock()
            .unwrap()
            .doPlayerClick(x, y, 3 - currPlayer);
        if (validClick) {
            let currMove = MAIN_GAME.lock().unwrap().Move;
            match currMove {
                1 | 3 => {
                    setStatus("Opponent's turn to expand");
                    setTitle(false);
                    playSFX("click.mp3");
                }
                0 | 2 => {
                    setStatus("Your turn to place");
                    setTitle(true);
                    playSFX("click.mp3");
                }
                _ => {}
            }
            render();
        }
    } else if (str == "Win") {
        let currPlayer = *Player.lock().unwrap();
        let win = MAIN_GAME.lock().unwrap().checkWin(x, y);
        if (win == 3 - currPlayer) {
            setStatus("You Lost, Press Shift to Start a New Game");
            setTitle(true);
            playSFX("finish.mp3");
            *Player.lock().unwrap() = 3 - currPlayer;
            MAIN_GAME.lock().unwrap().Move = -1;
            *OppGameStart.lock().unwrap() = 0;
            *PlayerGameStart.lock().unwrap() = 0;
        }
    } else if (str == "Resign") {
        let currPlayer = *Player.lock().unwrap();
        setStatus("Opponent Resigned, Press Shift to Start a New Game");
        setTitle(true);
        playSFX("finish.mp3");
        *Player.lock().unwrap() = 3 - currPlayer;
        MAIN_GAME.lock().unwrap().Move = -1;
        *OppGameStart.lock().unwrap() = 0;
        *PlayerGameStart.lock().unwrap() = 0;
    } else if (str == "Start") {
        if (MAIN_GAME.lock().unwrap().Move == -1) {
            if (*PlayerGameStart.lock().unwrap() == 0) {
                *OppGameStart.lock().unwrap() = 1;
                setStatus("Opponent is waiting for you to Start New Game, Press Shift to Start a New Game");
                setTitle(true);
                playSFX("click.mp3")
            } else if (*PlayerGameStart.lock().unwrap() == 1) {
                MAIN_GAME.lock().unwrap().Move = 0;
                reset();
            }
        }
    }
}
