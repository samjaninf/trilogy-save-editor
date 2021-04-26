use anyhow::*;
use indexmap::IndexMap;
use serde::Serialize;

use crate::{gui::Gui, save_data::Dummy};

use super::{
    common::{EndGameState, Level, Rotator, SaveTimeStamp, StreamingRecord, Vector},
    ImguiString, SaveCursor, SaveData,
};

pub mod player;
use player::*;

mod squad;
use squad::*;

pub mod plot;
use plot::*;

pub mod known_plot;

mod galaxy_map;
use galaxy_map::*;

#[derive(Serialize, SaveData, Clone)]
pub struct Me3SaveGame {
    _version: Version,
    _debug_name: ImguiString,
    seconds_played: f32,
    _disc: Dummy<4>,
    base_level_name: ImguiString,
    base_level_name_display_override_as_read: ImguiString,
    pub difficulty: Difficulty,
    pub end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotator,
    _current_loading_tip: Dummy<4>,
    levels: Vec<Level>,
    streaming_records: Vec<StreamingRecord>,
    _kismet_records: Vec<Dummy<20>>,
    _doors: Vec<Dummy<18>>,
    _placeables: Vec<Dummy<18>>,
    _pawns: Vec<Dummy<16>>,
    pub player: Player,
    squad: Vec<Henchman>,
    pub plot: PlotTable,
    _me1_plot: Me1PlotTable,
    pub player_variables: IndexMap<ImguiString, i32>,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    treasures: Vec<LevelTreasure>,
    _use_modules: Vec<Dummy<16>>,
    pub conversation_mode: AutoReplyModeOptions,
    objectice_markers: Vec<ObjectiveMarker>,
    _saved_objective_text: Dummy<4>,
}

#[derive(Serialize, Clone)]
pub struct Version(i32);

impl SaveData for Version {
    fn deserialize(cursor: &mut SaveCursor) -> Result<Self> {
        let version = SaveData::deserialize(cursor)?;

        ensure!(
            version == 59,
            "Wrong save version, please use a save from the last version of the game"
        );

        Ok(Self(version))
    }

    fn draw_raw_ui(&mut self, _: &Gui, _: &str) {}
}

#[derive(Serialize, SaveData, Clone)]
pub enum Difficulty {
    Narrative,
    Casual,
    Normal,
    Hardcore,
    Insanity,
}

#[derive(Serialize, SaveData, Default, Clone)]
struct DependentDlc {
    id: i32,
    name: ImguiString,
    canonical_name: ImguiString,
}

#[derive(Serialize, SaveData, Default, Clone)]
struct LevelTreasure {
    level_name: ImguiString,
    credits: i32,
    xp: i32,
    items: Vec<ImguiString>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize, SaveData, Clone)]
pub enum AutoReplyModeOptions {
    AllDecisions,
    MajorDecisions,
    NoDecisions,
}

#[derive(Serialize, SaveData, Default, Clone)]
struct ObjectiveMarker {
    marker_owned_data: ImguiString,
    marker_offset: Vector,
    marker_label: i32,
    bone_to_attach_to: ImguiString,
    marker_icon_type: ObjectiveMarkerIconType,
}

#[derive(Serialize, SaveData, Clone)]
enum ObjectiveMarkerIconType {
    None,
    Attack,
    Supply,
    Alert,
}

impl Default for ObjectiveMarkerIconType {
    fn default() -> Self {
        ObjectiveMarkerIconType::None
    }
}

#[cfg(test)]
mod test {
    use anyhow::*;
    use crc::{Crc, CRC_32_BZIP2};
    use std::{
        time::Instant,
        {fs::File, io::Read},
    };

    use crate::{save_data::*, unreal};

    use super::*;

    #[test]
    fn deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/ME3Save.pcsav")?;
            file.read_to_end(&mut input)?;
        }

        let now = Instant::now();

        // Deserialize
        let mut cursor = SaveCursor::new(input.clone());
        let me3_save_game = Me3SaveGame::deserialize(&mut cursor)?;

        println!("Deserialize : {:?}", Instant::now().saturating_duration_since(now));
        let now = Instant::now();

        // Serialize
        let mut output = unreal::Serializer::to_bytes(&me3_save_game)?;

        // Checksum
        let crc = Crc::<u32>::new(&CRC_32_BZIP2);
        let checksum = crc.checksum(&output);
        output.extend(&u32::to_le_bytes(checksum));

        println!("Serialize : {:?}", Instant::now().saturating_duration_since(now));

        // Check serialized = input
        let cmp = input.chunks(4).zip(output.chunks(4));
        for (i, (a, b)) in cmp.enumerate() {
            if a != b {
                panic!("0x{:02x?} : {:02x?} != {:02x?}", i * 4, a, b);
            }
        }

        Ok(())
    }
}
