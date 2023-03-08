use bevy_ecs_tilemap::{
    prelude::{TilemapSize, TilemapTileSize},
    tiles::TileTextureIndex,
};

// I'd like if this was dynamic based on screen size? Maybe. I dunno.
pub const MAP_SIZE: TilemapSize = TilemapSize { x: 24, y: 42 };

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

// The sprite sheet is 49x18
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum TileType {
    Ground = 0,
    GroundWithDirt = 1,
    GroundPathPartial = 2,
    GroundPathSmall = 3,
    GroundPathLarge = 4,
    GroundWithGrass = 5,
    GroundWithWeeds = 6,
    GroundWithLatticedGrass = 7,
    RoadVertical = 8,
    RoadRightBend = 9,
    RoadVerticalWithRightBend = 10,
    Road4Way = 11,
    RoadEnd = 12,
    Dunno1 = 13,
    Dunno2 = 14,
    Dunno3 = 15,
    SideWalk = 16,
    Dunno4 = 17,
    PlatformTopLeft = 18,
    PlatformTop = 19,
    PlatformTopRight = 20,
    LadderTop = 21,
    Spikes = 22,
    LockOutline = 23,
    HumanWithRobe = 24,
    Human = 25,
    HumanWithSpear = 26,
    HumanWithSwordAndShield = 27,
    HumanWithSwordAndShieldAndHelmet = 28,
    HumanWithSpearAndShieldAndHelmet = 29,
    HumanWithHeavyArmorAndHelmet = 30,
    HumanWithSwordAndShieldAndHornedHelmet = 31,
    HelmetWithCheekGuards1 = 32,
    HelmetWithCheekGuards2 = 33,
    HelmetWithCheekGuards3 = 34,
    HelmetWithCheekGuards4 = 35,
    HelmetWithCheekGuards5 = 36,
    HelmetRacing = 37,
    HelmetMining = 38,
    Dunno5 = 39,
    Dunno6 = 40,
    Gloves1 = 41,
    Gloves2 = 42,
    CarProfileLarge = 43,
    CarProfileSmall = 44,
    CarProfileConvertible = 45,
    CarPortraitLarge = 46,
    Dunno7 = 47,
    WheelChair = 48,
    // Row 2
    Tree = 49,
    TreePine = 50,
    TreeRound = 51,
    Trees = 52,
    LargeTree = 53,
    TreeWithLongTrunk = 54,
    Cactus = 55,
    Cactuses = 56,
    DirtRoadVertical = 57,
    DirtRoadRightBend = 58,
    DirtRoadVerticalWithRightBend = 59,
    DirtRoad4Way = 60,
    DirtRoadEnd = 61,
}

impl TileType {
    pub fn index(&self) -> TileTextureIndex {
        TileTextureIndex(*self as u32)
    }
}
