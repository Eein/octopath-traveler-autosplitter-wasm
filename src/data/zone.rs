use phf::phf_map;

pub struct ZoneDescription {
    name: &'static str,
    region: &'static str,
    ring: u8,
}

// pub struct RegionDescription {
//     name: &'static str,
//     ring: u8
// }

pub static ADVANCED_JOB_FIGHTS: phf::Map<u8, &'static str> = phf_map! {
    187_u8 => "Steorra",
    188_u8 => "Balogar",
    189_u8 => "Winnehild",
    190_u8 => "Dreisang"
};

pub static SHRINES: phf::Map<u8, &'static str> = phf_map! {
    179_u8 => "Cleric Shrine",
    180_u8 => "Scholar Shrine",
    181_u8 => "Merchant Shrine",
    182_u8 => "Warrior Shrine",
    183_u8 => "Dancer Shrine",
    184_u8 => "Apothecary Shrine",
    185_u8 => "Thief Shrine",
    186_u8 => "Hunter Shrine"
};

pub static AREAS: phf::Map<u8, ZoneDescription> = phf_map! {
    12_u8 => ZoneDescription{name: "Cobbleston", region: "Highlands", ring: 1},
    13_u8 => ZoneDescription{name: "Stonegard", region: "Highlands", ring: 2},
    14_u8 => ZoneDescription{name: "Stonegard Heights", region: "Highlands", ring: 2},
    15_u8 => ZoneDescription{name: "Stonegard Valleys", region: "Highlands", ring: 2},
    16_u8 => ZoneDescription{name: "Everhold", region: "Highlands", ring: 3},
    17_u8 => ZoneDescription{name: "Everhold Amphitheatre", region: "Highlands", ring: 3},
    18_u8 => ZoneDescription{name: "Mountain Pass", region: "Highlands", ring: 1},
    19_u8 => ZoneDescription{name: "North Cobbleston Gap", region: "Highlands", ring: 1},
    20_u8 => ZoneDescription{name: "South Cobbleston Gap", region: "Highlands", ring: 1},
    21_u8 => ZoneDescription{name: "Brigand's Den", region: "Highlands", ring: 1},
    22_u8 => ZoneDescription{name: "Untouched Sanctum", region: "Highlands", ring: 1},
    23_u8 => ZoneDescription{name: "Spectrewood Path", region: "Highlands", ring: 2},
    24_u8 => ZoneDescription{name: "North Stoneguard Pass", region: "Highlands", ring: 2},
    25_u8 => ZoneDescription{name: "West Stonegard Pass", region: "Highlands", ring: 2},
    26_u8 => ZoneDescription{name: "The Spectrewood", region: "Highlands", ring: 2},
    27_u8 => ZoneDescription{name: "Yvon's Cellar, Yvon's Birthplace", region: "Highlands", ring: 2},
    28_u8 => ZoneDescription{name: "Tomb of Kings", region: "Highlands", ring: 2},
    29_u8 => ZoneDescription{name: "West Everhold Pass", region: "Highlands", ring: 3},
    30_u8 => ZoneDescription{name: "Amphitheatre: Arena", region: "Highlands", ring: 3},
    31_u8 => ZoneDescription{name: "Amphitheatre: Balcony", region: "Highlands", ring: 3},
    32_u8 => ZoneDescription{name: "Everhold Amphitheatre deeper into Prim 4 -> end of Prim 4)", region: "Highlands",  ring: 3},
    33_u8 => ZoneDescription{name: "Everhold Tunnels", region: "Highlands", ring: 3},
    34_u8 => ZoneDescription{name: "Sunshade", region: "Sunlands", ring: 1},
    35_u8 => ZoneDescription{name: "Sunshade Tavern", region: "Sunlands", ring: 1},
    36_u8 => ZoneDescription{name: "Southern Sunshade Sands", region: "Sunlands", ring: 1},
    37_u8 => ZoneDescription{name: "Eastern Sunshade Sands", region: "Sunlands", ring: 1},
    38_u8 => ZoneDescription{name: "Sunshade Catacombs", region: "Sunlands", ring: 1},
    39_u8 => ZoneDescription{name: "Whistling Cavern", region: "Sunlands", ring: 1},
    40_u8 => ZoneDescription{name: "Wellspring", region: "Sunlands", ring: 2},
    42_u8 => ZoneDescription{name: "Western Wellspring Sands", region: "Sunlands", ring: 2},
    43_u8 => ZoneDescription{name: "Southern Wellspring Sands", region: "Sunlands", ring: 2},
    44_u8 => ZoneDescription{name: "Northern Wellspring Sands", region: "Sunlands", ring: 2},
    45_u8 => ZoneDescription{name: "Eastern Wellspring Sands", region: "Sunlands", ring: 2},
    46_u8 => ZoneDescription{name: "Lizardman's Den", region: "Sunlands", ring: 2},
    47_u8 => ZoneDescription{name: "Black Market", region: "Sunlands", ring: 2},
    48_u8 => ZoneDescription{name: "Quicksand Caves", region: "Sunlands", ring: 2},
    49_u8 => ZoneDescription{name: "Marsalim", region: "Sunlands", ring: 3},
    50_u8 => ZoneDescription{name: "Marsalim Palace", region: "Sunlands", ring: 3},
    51_u8 => ZoneDescription{name: "Grimsand Road", region: "Sunlands", ring: 3},
    52_u8 => ZoneDescription{name: "Eastern Marsalim Sands", region: "Sunlands", ring: 3},
    53_u8 => ZoneDescription{name: "Grimsand Ruins #1", region: "Sunlands", ring: 3},
    54_u8 => ZoneDescription{name: "Grimsand Ruins #2", region: "Sunlands", ring: 3},
    55_u8 => ZoneDescription{name: "Marsalim Catacombs", region: "Sunlands", ring: 3},
    56_u8 => ZoneDescription{name: "Clearbrook", region: "Riverlands", ring: 1},
    57_u8 => ZoneDescription{name: "Path of Rhiyo", region: "Riverlands", ring: 1},
    58_u8 => ZoneDescription{name: "South Clearbrook Traverse", region: "Riverlands", ring: 1},
    59_u8 => ZoneDescription{name: "West Clearbrook Traverse", region: "Riverlands", ring: 1},
    60_u8 => ZoneDescription{name: "Cave of Rhiyo", region: "Riverlands", ring: 1},
    61_u8 => ZoneDescription{name: "Twin Falls", region: "Riverlands", ring: 1},
    62_u8 => ZoneDescription{name: "Saintsbridge", region: "Riverlands", ring: 2},
    63_u8 => ZoneDescription{name: "Saintsbridge: Upstream", region: "Riverlands", ring: 2},
    64_u8 => ZoneDescription{name: "Saintsbridge Cathedral", region: "Riverlands", ring: 2},
    65_u8 => ZoneDescription{name: "Murkwood Trail", region: "Riverlands", ring: 2},
    66_u8 => ZoneDescription{name: "East Saintsbridge Traverse", region: "Riverlands", ring: 2},
    67_u8 => ZoneDescription{name: "The Murkwood", region: "Riverlands", ring: 2},
    68_u8 => ZoneDescription{name: "Rivira Woods", region: "Riverlands", ring: 2},
    69_u8 => ZoneDescription{name: "Farshore", region: "Riverlands", ring: 2},
    70_u8 => ZoneDescription{name: "Riverford", region: "Riverlands", ring: 3},
    71_u8 => ZoneDescription{name: "Manse Gardens, Lower Riverford", region: "Riverlands", ring: 3},
    72_u8 => ZoneDescription{name: "North Riverford Traverse", region: "Riverlands", ring: 3},
    73_u8 => ZoneDescription{name: "Hidden Path", region: "Riverlands", ring: 3},
    74_u8 => ZoneDescription{name: "Lord's Manse", region: "Riverlands", ring: 3},
    75_u8 => ZoneDescription{name: "Refuge Ruins", region: "Riverlands", ring: 3},
    76_u8 => ZoneDescription{name: "Atlasdam", region: "Flatlands", ring: 1},
    77_u8 => ZoneDescription{name: "Atlasdam Palace Gate, Royal Academy of Atlasdam", region: "Flatlands", ring: 1},
    78_u8 => ZoneDescription{name: "Atlasdam Palace", region: "Flatlands", ring: 1},
    79_u8 => ZoneDescription{name: "East Atlasdam Flats", region: "Flatlands", ring: 1},
    80_u8 => ZoneDescription{name: "North Atlasdam Flats", region: "Flatlands", ring: 1},
    81_u8 => ZoneDescription{name: "Subterranean Study", region: "Flatlands", ring: 1},
    82_u8 => ZoneDescription{name: "The Whistlewood", region: "Flatlands", ring: 1},
    83_u8 => ZoneDescription{name: "Noblecourt", region: "Flatlands", ring: 2},
    84_u8 => ZoneDescription{name: "East Noblecourt", region: "Flatlands", ring: 2},
    85_u8 => ZoneDescription{name: "Western Noblecourt Flats", region: "Flatlands", ring: 2},
    86_u8 => ZoneDescription{name: "Orlick's Manse", region: "Flatlands", ring: 2},
    87_u8 => ZoneDescription{name: "Obsidian Manse", region: "Flatlands", ring: 2},
    88_u8 => ZoneDescription{name: "The Hollow Throne", region: "Flatlands", ring: 2},
    89_u8 => ZoneDescription{name: "Wispermill", region: "Flatlands", ring: 3},
    90_u8 => ZoneDescription{name: "Western Wispermill Flats", region: "Flatlands", ring: 3},
    91_u8 => ZoneDescription{name: "Ebony Grotto #1", region: "Flatlands", ring: 3},
    92_u8 => ZoneDescription{name: "Ebony Grotto #2", region: "Flatlands", ring: 3},
    93_u8 => ZoneDescription{name: "Forest of Purgation", region: "Flatlands", ring: 3},
    94_u8 => ZoneDescription{name: "Bolderfall", region: "Clifflands", ring: 1},
    95_u8 => ZoneDescription{name: "Lower Bolderfall", region: "Clifflands", ring: 1},
    96_u8 => ZoneDescription{name: "Ruvus Manor Gate", region: "Clifflands", ring: 1},
    97_u8 => ZoneDescription{name: "North Bolderfall Pass", region: "Clifflands", ring: 1},
    98_u8 => ZoneDescription{name: "South Bolderfall Pass", region: "Clifflands", ring: 1},
    99_u8 => ZoneDescription{name: "Ravus Manor", region: "Clifflands", ring: 1},
    100_u8 => ZoneDescription{name: "Carrion Caves", region: "Clifflands", ring: 1},
    101_u8 => ZoneDescription{name: "Quarrycrest", region: "Clifflands", ring: 2},
    102_u8 => ZoneDescription{name: "Quarrycrest Mines", region: "Clifflands", ring: 2},
    103_u8 => ZoneDescription{name: "Road to Morlock's Manse", region: "Clifflands", ring: 2},
    104_u8 => ZoneDescription{name: "South Quarrycrest Pass", region: "Clifflands", ring: 2},
    105_u8 => ZoneDescription{name: "Morlock's Manse", region: "Clifflands", ring: 2},
    106_u8 => ZoneDescription{name: "The Sewers", region: "Clifflands", ring: 2},
    107_u8 => ZoneDescription{name: "Derelict Mine", region: "Clifflands", ring: 2},
    108_u8 => ZoneDescription{name: "Orewell", region: "Clifflands", ring: 3},
    109_u8 => ZoneDescription{name: "Trail to Forest of Rubeh", region: "Clifflands", ring: 3},
    110_u8 => ZoneDescription{name: "South Orewell Pass", region: "Clifflands", ring: 3},
    111_u8 => ZoneDescription{name: "Forest of Rubeh #1", region: "Clifflands", ring: 3},
    112_u8 => ZoneDescription{name: "Forest of Rubeh #2", region: "Clifflands", ring: 3},
    113_u8 => ZoneDescription{name: "Dragonsong Fane", region: "Clifflands", ring: 3},
    114_u8 => ZoneDescription{name: "Rippletide", region: "Coastlands", ring: 1},
    115_u8 => ZoneDescription{name: "Path to the Caves of Maiya", region: "Coastlands", ring: 1},
    116_u8 => ZoneDescription{name: "North Rippletide Coast", region: "Coastlands", ring: 1},
    117_u8 => ZoneDescription{name: "East Rippletide Coast", region: "Coastlands", ring: 1},
    118_u8 => ZoneDescription{name: "Caves of Maiya", region: "Coastlands", ring: 1},
    119_u8 => ZoneDescription{name: "Undertow Cave", region: "Coastlands", ring: 1},
    120_u8 => ZoneDescription{name: "Goldshore", region: "Coastlands", ring: 2},
    121_u8 => ZoneDescription{name: "Goldshore Manor District", region: "Coastlands", ring: 2},
    122_u8 => ZoneDescription{name: "Goldshore Cathedral", region: "Coastlands", ring: 2},
    123_u8 => ZoneDescription{name: "Road to the Seaside Grotto", region: "Coastlands", ring: 2},
    124_u8 => ZoneDescription{name: "Road to the Caves of Azure", region: "Coastlands", ring: 2},
    125_u8 => ZoneDescription{name: "West Goldshore Coast", region: "Coastlands", ring: 2},
    126_u8 => ZoneDescription{name: "Moonstruck Coast", region: "Coastlands", ring: 2},
    127_u8 => ZoneDescription{name: "Seaside Grotto", region: "Coastlands", ring: 2},
    128_u8 => ZoneDescription{name: "Caves of Azure", region: "Coastlands", ring: 2},
    129_u8 => ZoneDescription{name: "Captain's Bane", region: "Coastlands", ring: 2},
    130_u8 => ZoneDescription{name: "Grandport #1", region: "Coastlands", ring: 3},
    131_u8 => ZoneDescription{name: "Grandport #2", region: "Coastlands", ring: 3},
    132_u8 => ZoneDescription{name: "Grandport #3", region: "Coastlands", ring: 3},
    133_u8 => ZoneDescription{name: "West Grandport Coast", region: "Coastlands", ring: 3},
    134_u8 => ZoneDescription{name: "Grandport Sewers #1", region: "Coastlands", ring: 3},
    135_u8 => ZoneDescription{name: "Grandport Sewers #2", region: "Coastlands", ring: 3},
    136_u8 => ZoneDescription{name: "Loch of the Lost King", region: "Coastlands", ring: 3},
    137_u8 => ZoneDescription{name: "Flamesgrace #1", region: "Frostlands", ring: 1},
    138_u8 => ZoneDescription{name: "Flamesgrace #2", region: "Frostlands", ring: 1},
    139_u8 => ZoneDescription{name: "Flamesgrace Church", region: "Frostlands", ring: 1},
    140_u8 => ZoneDescription{name: "Path to the Cave of Origin", region: "Frostlands", ring: 1},
    141_u8 => ZoneDescription{name: "Western Flamesgrace Wilds", region: "Frostlands", ring: 1},
    142_u8 => ZoneDescription{name: "Northern Flamesgrace Wilds", region: "Frostlands", ring: 1},
    143_u8 => ZoneDescription{name: "Cave of Origin", region: "Frostlands", ring: 1},
    144_u8 => ZoneDescription{name: "Hoarfrost Grotto", region: "Frostlands", ring: 1},
    145_u8 => ZoneDescription{name: "Stillsnow", region: "Frostlands", ring: 2},
    146_u8 => ZoneDescription{name: "Trail to the Whitewood", region: "Frostlands", ring: 2},
    147_u8 => ZoneDescription{name: "Road to the Obsidian Parlor", region: "Frostlands", ring: 2},
    148_u8 => ZoneDescription{name: "Western Stillsnow Wilds", region: "Frostlands", ring: 2},
    149_u8 => ZoneDescription{name: "The Whitewood", region: "Frostlands", ring: 2},
    150_u8 => ZoneDescription{name: "Secret Path", region: "Frostlands", ring: 2},
    151_u8 => ZoneDescription{name: "Tomb of the Imperator", region: "Frostlands", ring: 2},
    152_u8 => ZoneDescription{name: "Northreach", region: "Frostlands", ring: 3},
    153_u8 => ZoneDescription{name: "Northreach: Lorn Cathedral", region: "Frostlands", ring: 3},
    154_u8 => ZoneDescription{name: "Southern Northreach Wilds", region: "Frostlands", ring: 3},
    155_u8 => ZoneDescription{name: "Lorn Cathedral: Cellars", region: "Frostlands", ring: 3},
    156_u8 => ZoneDescription{name: "Lorn Cathedral: Cellars #2", region: "Frostlands", ring: 3},
    157_u8 => ZoneDescription{name: "Maw of the Ice Dragon", region: "Frostlands", ring: 3},
    158_u8 => ZoneDescription{name: "S'warkii", region: "Woodlands", ring: 1},
    159_u8 => ZoneDescription{name: "Path to the Whisperwood", region: "Woodlands", ring: 1},
    160_u8 => ZoneDescription{name: "West S'warkii Trail", region: "Woodlands", ring: 1},
    161_u8 => ZoneDescription{name: "North S'warkii Trail", region: "Woodlands", ring: 1},
    162_u8 => ZoneDescription{name: "The Whisperwood", region: "Woodlands", ring: 1},
    163_u8 => ZoneDescription{name: "Path of Beasts", region: "Woodlands", ring: 1},
    164_u8 => ZoneDescription{name: "Victor's Hollow", region: "Woodlands", ring: 2},
    165_u8 => ZoneDescription{name: "Victor's Hollow: Arena Gate", region: "Woodlands", ring: 2},
    166_u8 => ZoneDescription{name: "Victor's Hollow: Arena", region: "Woodlands", ring: 2},
    167_u8 => ZoneDescription{name: "Path to the Forgotten Grotto", region: "Woodlands", ring: 2},
    168_u8 => ZoneDescription{name: "East Victor's Hollow Trail", region: "Woodlands", ring: 2},
    169_u8 => ZoneDescription{name: "Forgotten Grotto", region: "Woodlands", ring: 2},
    170_u8 => ZoneDescription{name: "Forest of No Return", region: "Woodlands", ring: 2},
    171_u8 => ZoneDescription{name: "Duskbarrow", region: "Woodlands", ring: 3},
    172_u8 => ZoneDescription{name: "East Duskbarrow Trail", region: "Woodlands", ring: 3},
    173_u8 => ZoneDescription{name: "Ruins of Eld #1", region: "Woodlands", ring: 3},
    174_u8 => ZoneDescription{name: "Ruins of Eld #2", region: "Woodlands", ring: 3},
    175_u8 => ZoneDescription{name: "Moldering Ruins", region: "Woodlands", ring: 3},
    179_u8 => ZoneDescription{name: "Shrine of the Flamebearer", region: "Frostlands", ring: 2},
    180_u8 => ZoneDescription{name: "Shrine of the Sage", region: "Flatlands", ring: 2},
    181_u8 => ZoneDescription{name: "Shrine of the Trader", region: "Coastlands", ring: 2},
    182_u8 => ZoneDescription{name: "Shrine of the Thunderblade", region: "Highlands", ring: 2},
    183_u8 => ZoneDescription{name: "Shrine of the Lady of Grace", region: "Sunlands", ring: 2},
    184_u8 => ZoneDescription{name: "Shrine of the Healer", region: "Frostlands", ring: 2},
    185_u8 => ZoneDescription{name: "Shrine of the Prince of Thieves", region: "Clifflands", ring: 2},
    186_u8 => ZoneDescription{name: "Shrine of the Huntress", region: "Woodlands", ring: 2},
    187_u8 => ZoneDescription{name: "Shrine of the Starseer", region: "Flatlands", ring: 3},
    188_u8 => ZoneDescription{name: "Shrine of the Runeblade", region: "Highlands", ring: 3},
    189_u8 => ZoneDescription{name: "Shrine of the Warbringer", region: "Riverlands", ring: 3},
    190_u8 => ZoneDescription{name: "Shrine of the Archmagus", region: "Woodlands", ring: 3},
    193_u8 => ZoneDescription{name: "Obsidian Parlor", region: "Frostlands", ring: 2},
    194_u8 => ZoneDescription{name: "Ruins of Hornburg", region: "Highlands", ring: 3},
    195_u8 => ZoneDescription{name: "The Gate of Finis", region: "Highlands", ring: 3},
    196_u8 => ZoneDescription{name: "Journey's End", region: "Highlands", ring: 3},
};
