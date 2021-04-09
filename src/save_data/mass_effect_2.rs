use imgui::ImString;

use crate::save_data::Dummy;

mod player;
use player::*;

mod squad;
use squad::*;

pub mod plot;
use plot::*;

mod galaxy_map;
use galaxy_map::*;

use super::common::{Checksum, EndGameState, Level, Rotation, SaveTimeStamp, StreamingRecord, Vector, Version};

#[derive(SaveData, Clone)]
pub struct Me2SaveGame {
    version: Version,
    debug_name: Vec<Dummy<1>>,
    seconds_played: f32,
    disc: Dummy<4>,
    base_level_name: ImString,
    difficulty: Difficulty,
    end_game_state: EndGameState,
    timestamp: SaveTimeStamp,
    location: Vector,
    rotation: Rotation,
    current_loading_tip: Dummy<4>,
    levels: Vec<Level>,
    streaming_records: Vec<StreamingRecord>,
    kismet_records: Vec<Dummy<20>>,
    doors: Vec<Dummy<18>>,
    pawns: Vec<Dummy<16>>,
    player: Player,
    squad: Vec<Henchman>,
    plot: PlotTable,
    me1_plot: Me1PlotTable,
    galaxy_map: GalaxyMap,
    dependant_dlcs: Vec<DependentDlc>,
    checksum: Checksum,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum Difficulty {
    Casual,
    Normal,
    Veteran,
    Hardcore,
    Insanity,
}

#[derive(SaveData, Default, Clone)]
struct DependentDlc {
    id: i32,
    name: ImString,
}

#[derive(SaveData, Default, Clone)]
struct LevelTreasure {
    level_name: ImString,
    credits: i32,
    xp: i32,
    items: Vec<ImString>,
}

#[allow(clippy::enum_variant_names)]
#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
enum AutoReplyModeOptions {
    AllDecisions,
    MajorDecisions,
    NoDecisions,
}

#[derive(SaveData, Default, Clone)]
struct ObjectiveMarker {
    marker_owned_data: ImString,
    marker_offset: Vector,
    marker_label: i32,
    bone_to_attach_to: ImString,
    marker_icon_type: ObjectiveMarkerIconType,
}

#[derive(FromPrimitive, ToPrimitive, SaveData, Clone)]
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
    use anyhow::Result;
    use tokio::{fs::File, io::AsyncReadExt};

    use crate::save_data::*;

    use super::*;

    #[tokio::test]
    async fn test_deserialize_serialize() -> Result<()> {
        let mut input = Vec::new();
        {
            let mut file = File::open("test/ME2Save.pcsav").await?;
            file.read_to_end(&mut input).await?;
        }

        // Deserialize
        let mut cursor = SaveCursor::new(input.clone());
        let me3_save_game = Me2SaveGame::deserialize(&mut cursor)?;

        // Serialize
        let mut output = Vec::new();
        Me2SaveGame::serialize(&me3_save_game, &mut output)?;

        // Check serialized = input
        assert_eq!(&output, &input);

        Ok(())
    }
}
