//! functionality relating to Minecraft biomes.

use num_enum::{IntoPrimitive, TryFromPrimitive};

// Values from https://minecraft.gamepedia.com/Java_Edition_data_value#Biomes
#[derive(TryFromPrimitive, IntoPrimitive, Debug)]
#[repr(i32)] // i32 as in corresponding NBT.
pub enum Biome {
    Ocean = 0,
    DeepOcean = 24,
    FrozenOcean = 10,
    DeepFrozenOcean = 50,
    ColdOcean = 46,
    DeepColdOcean = 49,
    LukewarmOcean = 45,
    DeepLukewarmOcean = 48,
    WarmOcean = 44,
    DeepWarmOcean = 47,
    River = 7,
    FrozenRiver = 11,
    Beach = 16,
    StoneShore = 25,
    SnowyBeach = 26,
    Forest = 4,
    WoodedHills = 18,
    FlowerForest = 132,
    BirchForest = 27,
    BirchForestHills = 28,
    TallBirchForest = 155,
    TallBirchHills = 156,
    DarkForest = 29,
    DarkForestHills = 157,
    Jungle = 21,
    JungleHills = 22,
    ModifiedJungle = 149,
    JungleEdge = 23,
    ModifiedJungleEdge = 151,
    BambooJungle = 168,
    BambooJungleHills = 169,
    Taiga = 5,
    TaigaHills = 19,
    TaigaMountains = 133,
    SnowyTaiga = 30,
    SnowyTaigaHills = 31,
    SnowyTaigaMountains = 158,
    GiantTreeTaiga = 32,
    GiantTreeTaigaHills = 33,
    GiantSpruceTaiga = 160,
    GiantSpruceTaigaHills = 161,
    MushroomFields = 14,
    MushroomFieldShore = 15,
    Swamp = 6,
    SwampHills = 134,
    Savanna = 35,
    SavannaPlateau = 36,
    ShatteredSavanna = 163,
    ShatteredSavannaPlateau = 164,
    Plains = 1,
    SunflowerPlains = 129,
    Desert = 2,
    DesertHills = 17,
    DesertLakes = 130,
    SnowyTundra = 12,
    SnowyMountains = 13,
    IceSpikes = 140,
    Mountains = 3,
    WoodedMountains = 34,
    GravellyMountains = 131,
    ModifiedGravellyMountains = 162,
    MountainEdge = 20,
    Badlands = 37,
    BadlandsPlateau = 39,
    ModifiedBadlandsPlateau = 167,
    WoodedBadlandsPlateau = 38,
    ModifiedWoodedBadlandsPlateau = 166,
    ErodedBadlands = 165,
    Nether = 8,
    TheEnd = 9,
    SmallEndIslands = 40,
    EndMidlands = 41,
    EndHighlands = 42,
    EndBarrens = 43,
    TheVoid = 127,
}

pub struct Climate {
    pub temperature: f64,
    pub rainfall: f64,
}

// Values from https://github.com/erich666/Mineways/blob/master/Win/biomes.cpp
pub fn climate(b: Biome) -> Climate {
    let climate = |t, r| Climate {
        temperature: t,
        rainfall: r,
    };

    match b {
        Biome::Ocean => climate(0.5, 0.5),
        Biome::Plains => climate(0.8, 0.4),
        Biome::Desert => climate(2.0, 0.0),
        Biome::Mountains => climate(0.2, 0.3),
        Biome::Forest => climate(0.7, 0.8),
        Biome::Taiga => climate(0.25, 0.8),
        Biome::Swamp => climate(0.8, 0.9),
        Biome::River => climate(0.5, 0.5),
        Biome::Nether => climate(2.0, 0.0),
        Biome::TheEnd => climate(0.5, 0.5),
        Biome::FrozenOcean => climate(0.0, 0.5),
        Biome::FrozenRiver => climate(0.0, 0.5),
        Biome::SnowyTundra => climate(0.0, 0.5),
        Biome::SnowyMountains => climate(0.0, 0.5),
        Biome::MushroomFields => climate(0.9, 1.0),
        Biome::MushroomFieldShore => climate(0.9, 1.0),
        Biome::Beach => climate(0.8, 0.4),
        Biome::DesertHills => climate(2.0, 0.0),
        Biome::WoodedHills => climate(0.7, 0.8),
        Biome::TaigaHills => climate(0.25, 0.8),
        Biome::MountainEdge => climate(0.2, 0.3),
        Biome::Jungle => climate(0.95, 0.9),
        Biome::JungleHills => climate(0.95, 0.9),
        Biome::JungleEdge => climate(0.95, 0.8),
        Biome::DeepOcean => climate(0.5, 0.5),
        Biome::StoneShore => climate(0.2, 0.3),
        Biome::SnowyBeach => climate(0.05, 0.3),
        Biome::BirchForest => climate(0.6, 0.6),
        Biome::BirchForestHills => climate(0.6, 0.6),
        Biome::DarkForest => climate(0.7, 0.8),
        Biome::SnowyTaiga => climate(-0.5, 0.4),
        Biome::SnowyTaigaHills => climate(-0.5, 0.4),
        Biome::GiantTreeTaiga => climate(0.3, 0.8),
        Biome::GiantTreeTaigaHills => climate(0.3, 0.8),
        Biome::WoodedMountains => climate(0.2, 0.3),
        Biome::Savanna => climate(1.2, 0.0),
        Biome::SavannaPlateau => climate(1.0, 0.0),
        Biome::Badlands => climate(2.0, 0.0),
        Biome::WoodedBadlandsPlateau => climate(2.0, 0.0),
        Biome::BadlandsPlateau => climate(2.0, 0.0),
        Biome::SmallEndIslands => climate(0.5, 0.5),
        Biome::EndMidlands => climate(0.5, 0.5),
        Biome::EndHighlands => climate(0.5, 0.5),
        Biome::EndBarrens => climate(0.5, 0.5),
        Biome::WarmOcean => climate(0.8, 0.5),
        Biome::LukewarmOcean => climate(0.8, 0.5),
        Biome::ColdOcean => climate(0.8, 0.5),
        Biome::DeepWarmOcean => climate(0.8, 0.5),
        Biome::DeepLukewarmOcean => climate(0.8, 0.5),
        Biome::DeepColdOcean => climate(0.8, 0.5),
        Biome::DeepFrozenOcean => climate(0.8, 0.5),
        Biome::TheVoid => climate(0.5, 0.5),
        Biome::SunflowerPlains => climate(0.8, 0.4),
        Biome::DesertLakes => climate(2.0, 0.0),
        Biome::GravellyMountains => climate(0.2, 0.3),
        Biome::FlowerForest => climate(0.7, 0.8),
        Biome::TaigaMountains => climate(0.25, 0.8),
        Biome::SwampHills => climate(0.8, 0.9),
        Biome::IceSpikes => climate(0.0, 0.5),
        Biome::ModifiedJungle => climate(0.95, 0.9),
        Biome::ModifiedJungleEdge => climate(0.95, 0.8),
        Biome::TallBirchForest => climate(0.6, 0.6),
        Biome::TallBirchHills => climate(0.6, 0.6),
        Biome::DarkForestHills => climate(0.7, 0.8),
        Biome::SnowyTaigaMountains => climate(-0.5, 0.4),
        Biome::GiantSpruceTaiga => climate(0.25, 0.8),
        Biome::GiantSpruceTaigaHills => climate(0.25, 0.8),
        Biome::ModifiedGravellyMountains => climate(0.2, 0.3),
        Biome::ShatteredSavanna => climate(1.1, 0.0),
        Biome::ShatteredSavannaPlateau => climate(1.0, 0.0),
        Biome::ErodedBadlands => climate(2.0, 0.0),
        Biome::ModifiedWoodedBadlandsPlateau => climate(2.0, 0.0),
        Biome::ModifiedBadlandsPlateau => climate(2.0, 0.0),
        Biome::BambooJungle => climate(0.95, 0.9),
        Biome::BambooJungleHills => climate(0.95, 0.9),
        //  Biome::SoulSandValley => climate(2.0, 0.0),
        //  Biome::CrimsonForest => climate(2.0, 0.0),
        //  Biome::WarpedForest => climate(2.0, 0.0),
        //  Biome::BasaltDeltas => climate(2.0, 0.0),
    }
}
