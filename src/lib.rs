// #![no_std]
use spinning_top::{const_spinlock, Spinlock};

use bytemuck::{Pod};

use asr::{
    timer::{self, TimerState},
    watcher::Pair,
    Process,
    Address,
};

static STATE: Spinlock<State> = const_spinlock(State { game: None });

struct Watcher<T> {
    watcher: asr::watcher::Watcher<T>,
    address: Vec<u64>,
}

impl<T: Pod> Watcher<T> {
    fn new(address: Vec<u64>) -> Self {
        Self {
            watcher: asr::watcher::Watcher::new(),
            address,
        }
    }

    fn update(&mut self, process: &Process, module: u64) -> Option<&Pair<T>> {
        let value = process.read_pointer_path64::<T>(module, &self.address);
        self.watcher.update(value.ok())
    }
}

struct Game {
    process: Process,
    module: u64,
    start: Watcher<u8>,
}

impl Game {
    fn new(process: Process, module: u64) -> Option<Self> {
        let game = Self {
            process: process,
            module: module,
            start: Watcher::new(vec![0x2B32C48, 0xE30]),
        };
        Some(game)
    }

    fn update_vars(&mut self) -> Option<Vars<'_>> {
        Some(Vars {
            start: self.start.update(&self.process, self.module)?,
        })
    }
}

pub struct State {
    game: Option<Game>,
}

struct Vars<'a> {
    start: &'a Pair<u8>,
}

#[no_mangle]
pub extern "C" fn update() {
    let mut state = STATE.lock();
    if state.game.is_none() {
        match Process::attach("Octopath_Travel") {
            Some(process) => {
                match process.get_module("Octopath_Traveler-Win64-Shipping.exe") {
                    Ok(Address(module)) => {
                        state.game = Game::new(process, module)
                    },
                    _ => ()
                };
            },
            None => ()
        }
    }

    if let Some(game) = &mut state.game {
        if !game.process.is_open() {
            state.game = None;
            // timer::reset()
            return;
        }
        if let Some(vars) = game.update_vars() {

            match timer::state() {
                TimerState::NotRunning => {
                    asr::print_message("old");
                    asr::print_message(&vars.start.old.to_string());
                    asr::print_message("current");
                    asr::print_message(&vars.start.current.to_string());
                    if vars.start.old == 0 && vars.start.current == 1 {
                        timer::start()
                    }
                }
                TimerState::Running | TimerState::Paused => {
                }
                _ => {}
            }
        }
    }
}

