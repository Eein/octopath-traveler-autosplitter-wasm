// #![no_std]
use spinning_top::{const_spinlock, Spinlock};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

use bytemuck::Pod;

use asr::{
    timer::{self, TimerState},
    watcher::Pair,
    Address, Process,
};

mod data;
use data::zone::{ADVANCED_JOB_FIGHTS, AREAS, SHRINES};

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
    start: Watcher<u8>,
    zone_id: Watcher<u8>,
    money: Watcher<u32>,
    game_state: Watcher<u8>,
    cutscene_script_index: Watcher<u16>,
    cutscene_progress_bar: Watcher<f64>,
    ophilia_progress: Watcher<u16>,
    cyrus_progress: Watcher<u16>,
    tressa_progress: Watcher<u16>,
    olberic_progress: Watcher<u16>,
    primrose_progress: Watcher<u16>,
    alfyn_progress: Watcher<u16>,
    therion_progress: Watcher<u16>,
    haanit_progress: Watcher<u16>,
    encounters: u16,
    deaths: u16,
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
            encounters: 0,
            deaths: 0,
            flags: Default::default(),
            splits: HashSet::new(),
        };
        Some(game)
    }

    fn update_vars(&mut self) -> Option<Vars<'_>> {
        Some(Vars {
            start: self.start.update(&self.process, self.module)?,
            zone_id: self.zone_id.update(&self.process, self.module)?,
            money: self.money.update(&self.process, self.module)?,
            game_state: self.game_state.update(&self.process, self.module)?,
            cutscene_script_index: self
                .cutscene_script_index
                .update(&self.process, self.module)?,
            cutscene_progress_bar: self
                .cutscene_progress_bar
                .update(&self.process, self.module)?,
            ophilia_progress: self.ophilia_progress.update(&self.process, self.module)?,
            cyrus_progress: self.cyrus_progress.update(&self.process, self.module)?,
            tressa_progress: self.tressa_progress.update(&self.process, self.module)?,
            olberic_progress: self.olberic_progress.update(&self.process, self.module)?,
            primrose_progress: self.primrose_progress.update(&self.process, self.module)?,
            alfyn_progress: self.alfyn_progress.update(&self.process, self.module)?,
            therion_progress: self.therion_progress.update(&self.process, self.module)?,
            haanit_progress: self.haanit_progress.update(&self.process, self.module)?,
            encounters: &mut self.encounters,
            deaths: &mut self.deaths,
            flags: &mut self.flags,
            splits: &mut self.splits,
        })
    }
}

pub struct State {
    game: Option<Game>,
}

#[derive(Default, PartialEq)]
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

impl Display for Character {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Character::NoCharacter => write!(f, "none"),
            Character::Ophilia => write!(f, "ophilia"),
            Character::Cyrus => write!(f, "cyrus"),
            Character::Tressa => write!(f, "tressa"),
            Character::Olberic => write!(f, "olberic"),
            Character::Primrose => write!(f, "primrose"),
            Character::Alfyn => write!(f, "alfyn"),
            Character::Therion => write!(f, "therion"),
            Character::Haanit => write!(f, "haanit"),
        }
    }
}

#[derive(Default)]
pub struct Flags {
    char_chapter_ending: Character,
}

#[allow(unused)]
struct Vars<'a> {
    start: &'a Pair<u8>,
    zone_id: &'a Pair<u8>,
    money: &'a Pair<u32>,
    game_state: &'a Pair<u8>,
    cutscene_script_index: &'a Pair<u16>,
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
    encounters: &'a mut u16,
    deaths: &'a mut u16,
    splits: &'a mut HashSet<String>,
}

impl Vars<'_> {
    fn split(&mut self, key: &str) -> Option<String> {
        if self.splits.contains(key) {
            return None;
        }
        self.splits.insert(key.to_string());
        // we should be returning settings if they exist here to match if the user
        // has assigned a setting, otherwise return None;
        // return settings[key];
        //
        // These are my personal ophilia splits, i think the enters are correct, but i'll play it
        // by ear
        // let res = match key {
        //     "fight_guardian" => Some(key.to_string()),
        //     "character_cyrus" => Some(key.to_string()),
        //     "fight_russell" => Some(key.to_string()),
        //     "enter_83" => Some(key.to_string()),
        //     "character_tressa" => Some(key.to_string()),
        //     "fight_mikk_makk" => Some(key.to_string()),
        //     "get_warrior_shrine" => Some(key.to_string()),
        //     "merchant_shrine" => Some(key.to_string()),
        //     "enter_120" => Some(key.to_string()),
        //     "enter_130" => Some(key.to_string()),
        //     "enter_34" => Some(key.to_string()),
        //     "fight_hrodvitnir" => Some(key.to_string()),
        //     "fight_mm_sf" => Some(key.to_string()),
        //     "fight_cultists" => Some(key.to_string()),
        //     "fight_mattias" => Some(key.to_string()),
        //     "chapter_end_ophilia" => Some(key.to_string()),
        //     _ => None,
        // };
        // res
        Some(key.to_string())
    }

    fn name_to_key(&self, name: &str) -> String {
        return name.to_lowercase().replace(" ", "_").replace("'", "");
    }

    fn split_chapter(&mut self, progress: u16) -> Option<String> {
        if progress % 1000 == 0 && self.game_state.current == 2 && self.game_state.old == 5 {
            let current_chapter = progress / 1000;
            if current_chapter == 0 {
                return None;
            }
            let key = format!(
                "chapter_end_{}_{}",
                self.flags.char_chapter_ending,
                current_chapter.to_string()
            );
            self.flags.char_chapter_ending = Character::NoCharacter;
            return self.split(&key);
        }
        None
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
            return;
        }
        if let Some(mut vars) = game.update_vars() {
            timer::set_variable_int("Encounters", *vars.encounters);
            timer::set_variable_int("Deaths", *vars.deaths);
            match timer::state() {
                TimerState::NotRunning => {
                    if vars.start.old == 0 && vars.start.current == 1 {
                        *vars.deaths = 0;
                        *vars.encounters = 0;
                        *vars.splits = Default::default();
                        *vars.flags = Default::default();
                        timer::start()
                    }
                }
                TimerState::Running => {
                    if let Some(reason) = should_split(&mut vars) {
                        asr::print_message(&reason);
                        timer::split();
                    }
                    if vars.game_state.current == 6 && vars.game_state.old == 2 { 

                        // asr::print_message("incremementing encounters");
                        // asr::print_message("before:");
                        // asr::print_message(&vars.encounters.to_string());
                        *vars.encounters = *vars.encounters + 1;
                        // asr::print_message("after:");
                        // asr::print_message(&vars.encounters.to_string());
                    } 
                    if vars.game_state.current == 7 && vars.game_state.old == 6 { 
                        // asr::print_message("incrementing deaths");
                        // asr::print_message("before:");
                        // asr::print_message(&vars.deaths.to_string());
                        *vars.deaths = *vars.deaths + 1;
                        // asr::print_message("after:");
                        // asr::print_message(&vars.deaths.to_string());
                    }
                }
                _ => {}
            }
        }

    }
}

fn should_split(vars: &mut Vars) -> Option<String> {
    // TODO: we may not need to contains_key here on these lookup tables
    // Shrines
    if SHRINES.contains_key(&vars.zone_id.current)
        && vars.game_state.current == 5
        && vars.game_state.old == 2
    {
        if let Some(shrine) = SHRINES.get(&vars.zone_id.current) {
            let key = format!("get_{}", vars.name_to_key(&shrine));
            return vars.split(&key);
        }
    }

    // Advanced Job Fights
    if ADVANCED_JOB_FIGHTS.contains_key(&vars.zone_id.current)
        && vars.game_state.current == 5
        && vars.game_state.old == 6
    {
        if let Some(advanced_job) = ADVANCED_JOB_FIGHTS.get(&vars.zone_id.current) {
            let key = format!("advanced_job_fight_{}", vars.name_to_key(&advanced_job));
            return vars.split(&key);
        }
    }

    // Enter & Exit Area
    if vars.zone_id.old != vars.zone_id.current && vars.zone_id.old != 0 {
        // Enter Area
        if AREAS.contains_key(&vars.zone_id.current)
            && vars.game_state.current == 2
            && vars.game_state.old == 2
        {
            let key = format!("enter_{}", vars.zone_id.current.to_string());
            return vars.split(&key);
        }
        // Exit Area
        if AREAS.contains_key(&vars.zone_id.old)
            && (vars.game_state.old == 2 || vars.game_state.old == 4)
        {
            let key = format!("exit_{}", vars.zone_id.old.to_string());
            return vars.split(&key);
        }
    }

    // Characters Joining
    if vars.ophilia_progress.old == 0 && vars.ophilia_progress.current >= 120 {
        return vars.split("character_ophilia");
    }
    if vars.cyrus_progress.old == 0 && vars.cyrus_progress.current >= 100 {
        return vars.split("character_cyrus");
    }
    if vars.tressa_progress.old == 0 && vars.tressa_progress.current >= 110 {
        return vars.split("character_tressa");
    }
    if vars.olberic_progress.old == 0 && vars.olberic_progress.current >= 110 {
        return vars.split("character_olberic");
    }
    if vars.primrose_progress.old == 0 && vars.primrose_progress.current >= 140 {
        return vars.split("character_primrose");
    }
    if vars.alfyn_progress.old == 0 && vars.alfyn_progress.current >= 70 {
        return vars.split("character_alfyn");
    }
    if vars.haanit_progress.old == 0 && vars.haanit_progress.current >= 110 {
        return vars.split("character_haanit");
    }
    if vars.therion_progress.old == 0 && vars.therion_progress.current >= 70 {
        return vars.split("character_therion");
    }

    // Ophilia
    if vars.ophilia_progress.old < vars.ophilia_progress.current && vars.zone_id.old != 0 {
        if vars.ophilia_progress.current == 170 {
            return vars.split("fight_guardian");
        } else if vars.ophilia_progress.current == 1140 {
            return vars.split("fight_hrodvitnir");
        } else if vars.ophilia_progress.current == 2110 {
            return vars.split("fight_mm_sf");
        } else if vars.ophilia_progress.current == 3090 {
            return vars.split("fight_cultists");
        } else if vars.ophilia_progress.current == 3150 {
            return vars.split("fight_mattias");
        } else if vars.ophilia_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Ophilia;
        }
    }

    // Ophilia Ending
    if vars.ophilia_progress.current == 3160
        && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 94)
    {
        // will taking the 1 - the current + a sleep here freeze the timer?
        // could we use this to get accurate precision on a probable split?
        return vars.split("character_story_endings_ophilia");
    }

    // Cyrus
    if vars.cyrus_progress.old != vars.cyrus_progress.current && vars.zone_id.old != 0 {
        if vars.cyrus_progress.current == 130 {
            return vars.split("fight_russell");
        } else if vars.cyrus_progress.current == 1110 {
            return vars.split("fight_gideon");
        } else if vars.cyrus_progress.current == 2160 {
            return vars.split("fight_yvon");
        } else if vars.cyrus_progress.current == 3060 {
            return vars.split("fight_lucia");
        } else if vars.cyrus_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Cyrus;
        }
    }

    // Cyrus Ending
    if vars.cyrus_progress.current == 3110
        && vars.cutscene_script_index.current >= 138
        && vars.cutscene_progress_bar.current > 0.98
    {
        return vars.split("character_story_endings_cyrus");
    }

    // Tressa
    if vars.tressa_progress.old != vars.tressa_progress.current && vars.zone_id.old != 0 {
        if vars.tressa_progress.current == 170 {
            return vars.split("fight_mikk_makk");
        } else if vars.tressa_progress.current == 1120 {
            return vars.split("fight_omar");
        } else if vars.tressa_progress.current == 2150 {
            return vars.split("fight_venomtooth_tiger");
        } else if vars.tressa_progress.current == 3120 {
            return vars.split("fight_esmeralda");
        } else if vars.tressa_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Tressa;
        }
    }

    // Tressa Ending
    if vars.tressa_progress.current == 3180
        && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 209)
    {
        return vars.split("character_story_endings_tressa");
    }

    // Primrose
    if vars.primrose_progress.old != vars.primrose_progress.current && vars.zone_id.old != 0 {
        if vars.primrose_progress.current == 160 {
            return vars.split("fight_helgenish");
        } else if vars.primrose_progress.current == 1180 {
            return vars.split("fight_rufus");
        } else if vars.primrose_progress.current == 2170 {
            return vars.split("fight_albus");
        } else if vars.primrose_progress.current == 3150 {
            return vars.split("fight_simeon2");
        } else if vars.primrose_progress.current == 3120 {
            return vars.split("fight_simeon1");
        } else if vars.primrose_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Primrose;
        }
    }

    // Primrose Ending
    if vars.primrose_progress.current == 3150
        && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 94)
    {
        return vars.split("character_story_endings_primrose");
    }

    // Olberic
    if vars.olberic_progress.old < vars.olberic_progress.current && vars.zone_id.old != 0 {
        if vars.olberic_progress.current == 110 {
            return vars.split("fight_brigands1");
        } else if vars.olberic_progress.current == 140 {
            return vars.split("fight_brigands2");
        } else if vars.olberic_progress.current == 160 {
            return vars.split("fight_gaston");
        } else if vars.olberic_progress.current == 1070 {
            return vars.split("fight_victorino");
        } else if vars.olberic_progress.current == 1140 {
            return vars.split("fight_joshua");
        } else if vars.olberic_progress.current == 1180 {
            return vars.split("fight_archibold");
        } else if vars.olberic_progress.current == 1220 {
            return vars.split("fight_gustav");
        } else if vars.olberic_progress.current == 2070 {
            return vars.split("fight_lizards1");
        } else if vars.olberic_progress.current == 2080 {
            return vars.split("fight_lizards2");
        } else if vars.olberic_progress.current == 2110 {
            return vars.split("fight_lizardking");
        } else if vars.olberic_progress.current == 2130 {
            return vars.split("fight_erhardt");
        } else if vars.olberic_progress.current == 3050 {
            return vars.split("fight_red_hat");
        } else if vars.olberic_progress.current == 3110 {
            return vars.split("fight_werner");
        } else if vars.olberic_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Olberic;
        }
    }

    // Olberic Ending
    if vars.olberic_progress.current == 3120
        && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 174)
    {
        return vars.split("character_story_endings_olberic");
    }

    // Alfyn
    if vars.alfyn_progress.old != vars.alfyn_progress.current && vars.zone_id.old != 0 {
        if vars.alfyn_progress.current == 90 {
            return vars.split("fight_blotted_viper");
        } else if vars.alfyn_progress.current == 1130 {
            return vars.split("fight_vanessa");
        } else if vars.alfyn_progress.current == 2140 {
            return vars.split("fight_miguel");
        } else if vars.alfyn_progress.current == 3240 {
            return vars.split("fight_ogre_eagle");
        } else if vars.alfyn_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Alfyn;
        }
    }

    // Alfyn Ending
    if vars.alfyn_progress.current == 3300
        && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 93)
    {
        return vars.split("character_story_endings_alfyn");
    }

    // Therion
    if vars.therion_progress.old != vars.therion_progress.current && vars.zone_id.old != 0 {
        if vars.therion_progress.current == 140 {
            return vars.split("fight_heathecote");
        } else if vars.therion_progress.current == 1130 {
            return vars.split("fight_orlick");
        } else if vars.therion_progress.current == 2100 {
            return vars.split("fight_darius_henchmen");
        } else if vars.therion_progress.current == 2150 {
            return vars.split("fight_gareth");
        } else if vars.therion_progress.current == 3040 {
            return vars.split("fight_darius_underlings");
        } else if vars.therion_progress.current == 3140 {
            return vars.split("3_percent_steal");
        } else if vars.therion_progress.current == 3180 {
            return vars.split("fight_darius");
        } else if vars.therion_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Therion;
        }
    }

    // Therion Ending
    if vars.therion_progress.current == 3200
        && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 275)
    {
        return vars.split("character_story_endings_therion");
    }

    // H'aanit
    if vars.haanit_progress.old != vars.haanit_progress.current && vars.zone_id.old != 0 {
        if vars.haanit_progress.current == 110 {
            return vars.split("fight_ghisarma");
        } else if vars.haanit_progress.current == 1050 {
            return vars.split("fight_nathans_bodyguard");
        } else if vars.haanit_progress.current == 1100 {
            return vars.split("fight_ancient_one");
        } else if vars.haanit_progress.current == 1120 {
            return vars.split("fight_lord_of_the_forest");
        } else if vars.haanit_progress.current == 2030 {
            return vars.split("fight_alaic");
        } else if vars.haanit_progress.current == 2090 {
            return vars.split("fight_dragon");
        } else if vars.haanit_progress.current == 3130 {
            return vars.split("fight_redeye");
        } else if vars.haanit_progress.current % 1000 == 0 {
            vars.flags.char_chapter_ending = Character::Haanit;
        }
    }

    // H'aanit Ending
    if vars.haanit_progress.current == 3140
        && (vars.cutscene_progress_bar.current > 0.98 || vars.cutscene_script_index.current > 195)
    {
        return vars.split("character_story_endings_haanit");
    }

    // All Character Chapter Ends
    if vars.flags.char_chapter_ending != Character::NoCharacter
        && vars.game_state.current == 2
        && vars.game_state.old == 5
    {
        let progress = match vars.flags.char_chapter_ending {
            Character::NoCharacter => 0,
            Character::Ophilia => vars.ophilia_progress.current,
            Character::Cyrus => vars.cyrus_progress.current,
            Character::Tressa => vars.tressa_progress.current,
            Character::Olberic => vars.olberic_progress.current,
            Character::Primrose => vars.primrose_progress.current,
            Character::Alfyn => vars.alfyn_progress.current,
            Character::Therion => vars.alfyn_progress.current,
            Character::Haanit => vars.haanit_progress.current,
        };
        return vars.split_chapter(progress);
    }

    // Credits
    if vars.zone_id.current == 10 && vars.zone_id.current != vars.zone_id.old {
        return vars.split("credits");
    }
    // Galdera Splits
    else if vars.zone_id.current == 195 && vars.zone_id.old == 194 {
        return vars.split("finis_start");
    } else if vars.zone_id.current == 196 && vars.zone_id.old == 195 {
        return vars.split("journeys_end_start");
    } else if vars.zone_id.current == 196
        && vars.game_state.current == 6
        && vars.game_state.old == 5
    {
        return vars.split("galdera_phase_1_start");
    } else if vars.zone_id.current == 198 && vars.zone_id.old == 196 {
        return vars.split("galdera_phase_1_end");
    } else if vars.zone_id.current == 198
        && vars.zone_id.old == 198
        && vars.game_state.current == 6
        && vars.game_state.old == 5
    {
        return vars.split("galdera_phase_2_start");
    } else if vars.zone_id.current == 198
        && vars.zone_id.old == 198
        && vars.game_state.current == 5
        && vars.game_state.old == 6
    {
        return vars.split("galdera_phase_2_end");
    } else if vars.zone_id.current == 194 && vars.money.current - vars.money.old == 100000 {
        return vars.split("at_journeys_end");
    }

    None
}
