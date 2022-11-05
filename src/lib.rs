// #![no_std]
use spinning_top::{const_spinlock, Spinlock};
use std::collections::HashMap;

use bytemuck::Pod;

use asr::{
    timer::{self, TimerState},
    watcher::Pair,
    Address, Process,
};

mod data;
use data::zone::ZONES;

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
    zone_id: Watcher<u8>,
    money: Watcher<u16>,
    game_state: Watcher<u8>,
    cutscene_script_index: Watcher<u8>,
    cutscene_progress_bar: Watcher<f64>,
    ophilia_progress: Watcher<u8>,
    cyrus_progress: Watcher<u8>,
    tressa_progress: Watcher<u8>,
    olberic_progress: Watcher<u8>,
    primrose_progress: Watcher<u8>,
    alfyn_progress: Watcher<u8>,
    therion_progress: Watcher<u8>,
    haanit_progress: Watcher<u8>,
}

impl Game {
    fn new(process: Process, module: u64) -> Option<Self> {
        let game = Self {
            process,
            module,
            start: Watcher::new(vec![0x2B32C48, 0xE30]),
            zone_id: Watcher::new(vec![0x289D240, 0x36C]),
            money: Watcher::new(vec![0x0289CC48, 0x370, 0x158]),
            game_state: Watcher::new(vec![0x0289D270, 0x36C]),
            cutscene_script_index: Watcher::new(vec![0x289D230, 0x388]),
            cutscene_progress_bar: Watcher::new(vec![0x0289D268, 0x378, 0x20, 0x230, 0x288]),
            ophilia_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x510]),
            cyrus_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x1f0]),
            tressa_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x128]),
            olberic_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x60]),
            primrose_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x2b8]),
            alfyn_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x5d8]),
            therion_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x448]),
            haanit_progress: Watcher::new(vec![0x0289CC48, 0x370, 0x1C8, 0x380]),
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
                    Ok(Address(module)) => state.game = Game::new(process, module),
                    _ => (),
                };
            }
            None => (),
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
                    if vars.start.old == 0 && vars.start.current == 1 {
                        timer::start()
                    }
                }
                TimerState::Running | TimerState::Paused => {}
                _ => {}
            }
        }
    }
}
