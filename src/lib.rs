// #![no_std]
use spinning_top::{const_spinlock, Spinlock};
use std::collections::HashSet;

use bytemuck::Pod;

use asr::{
    timer::{self, TimerState},
    watcher::Pair,
    Address, Process,
};

mod data;
// use data::zone::{AREAS, SHRINES};

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
    splits: HashSet<String>,
    // is_chapter_ending: bool,
    // char_chapter_ending: String,
    start: Watcher<u8>,
    zone_id: Watcher<u8>,
    money: Watcher<u16>,
    game_state: Watcher<u8>,
    cutscene_script_index: Watcher<u8>,
    cutscene_progress_bar: Watcher<f64>,
    ophilia_progress: Watcher<u16>,
    cyrus_progress: Watcher<u16>,
    tressa_progress: Watcher<u16>,
    olberic_progress: Watcher<u16>,
    primrose_progress: Watcher<u16>,
    alfyn_progress: Watcher<u16>,
    therion_progress: Watcher<u16>,
    haanit_progress: Watcher<u16>,
    flags: Flags,
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
            flags: Default::default(),
            splits: HashSet::new()
        };
        Some(game)
    }

    fn update_vars(&mut self) -> Option<Vars<'_>> {
        Some(Vars {
            start: self.start.update(&self.process, self.module)?,
            zone_id: self.zone_id.update(&self.process, self.module)?,
            money: self.money.update(&self.process, self.module)?,
            game_state: self.game_state.update(&self.process, self.module)?,
            cutscene_script_index: self.cutscene_script_index.update(&self.process, self.module)?,
            cutscene_progress_bar: self.cutscene_progress_bar.update(&self.process, self.module)?,
            ophilia_progress: self.ophilia_progress.update(&self.process, self.module)?,
            cyrus_progress: self.cyrus_progress.update(&self.process, self.module)?,
            tressa_progress: self.tressa_progress.update(&self.process, self.module)?,
            olberic_progress: self.olberic_progress.update(&self.process, self.module)?,
            primrose_progress: self.primrose_progress.update(&self.process, self.module)?,
            alfyn_progress: self.alfyn_progress.update(&self.process, self.module)?,
            therion_progress: self.therion_progress.update(&self.process, self.module)?,
            haanit_progress: self.haanit_progress.update(&self.process, self.module)?,
            flags: &mut self.flags,
            splits: &mut self.splits,
        })
    }

}

pub struct State {
    game: Option<Game>,
}

#[derive(Default)]
pub enum Character {
    #[default]
    NoCharacter,
    Ophilia,
    Cyrus,
    Tressa,
    Olberic,
    Primrose,
    Alfyn,
    Therion,
    Haanit,
}

#[derive(Default)]
pub struct Flags {
    is_chapter_ending: bool,
    char_chapter_ending: Character,
}

#[allow(unused)]
struct Vars<'a> {
    start: &'a Pair<u8>,
    zone_id: &'a Pair<u8>,
    money: &'a Pair<u16>,
    game_state: &'a Pair<u8>,
    cutscene_script_index: &'a Pair<u8>,
    cutscene_progress_bar: &'a Pair<f64>,
    ophilia_progress: &'a Pair<u16>,
    cyrus_progress: &'a Pair<u16>,
    tressa_progress: &'a Pair<u16>,
    olberic_progress: &'a Pair<u16>,
    primrose_progress: &'a Pair<u16>,
    alfyn_progress: &'a Pair<u16>,
    therion_progress: &'a Pair<u16>,
    haanit_progress: &'a Pair<u16>,
    flags: &'a mut Flags,
    splits: &'a mut HashSet<String>,
}

impl Vars<'_> {
    fn split(&mut self, key: &str) -> Option<String> {
        if self.splits.contains(key) { return None; }
        self.splits.insert(key.to_string());
        // we should be returning settings if they exist here to match if the user
        // has assigned a setting, otherwise return None;
        // return settings[key];
        Some(key.to_string())
    }
}

pub struct Splits(HashSet<String>);

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
        let vars = game.update_vars();
        if let Some(mut vars) = vars {
            match timer::state() {
                TimerState::NotRunning => {
                    if vars.start.old == 0 && vars.start.current == 1 {
                        timer::start()
                    }
                }
                TimerState::Running => {
                    if let Some(reason) = should_split(&mut vars) {
                        asr::print_message(&reason);
                        timer::split();
                    }
                }
                _ => {}
            }
        }
    }
}

fn should_split(vars: &mut Vars) -> Option<String> {
    // Ophilia
    if vars.ophilia_progress.old < vars.ophilia_progress.current && vars.zone_id.old != 0 {
        match vars.ophilia_progress.current {
            170 => return vars.split("fight_guardian"),
            1140 => return vars.split("fight_hrodvitnir"),
            2110 => return vars.split("fight_mm_sf"),
            3090 => return vars.split("fight_cultists"),
            3150 => return vars.split("fight_mattias"),
            _ => ()
        }
        if vars.ophilia_progress.current % 1000 == 0 {
            vars.flags.is_chapter_ending = true;
            vars.flags.char_chapter_ending = Character::Ophilia;
        }
    }

    // Ophilia Ending
    if vars.ophilia_progress.current == 3160 && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 94) {
        // will taking the 1 - the current + a sleep here freeze the timer?
        // could we use this to get accurate precision on a probable split?
        return vars.split("character_story_endings_ophilia");
    }

    None
}
