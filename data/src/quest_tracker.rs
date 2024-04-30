
#[allow(non_snake_case)]
mod decl {
    use std::ffi::CString;

    use bitvec::prelude::*;

    pub const QUEST_TRACKER_FRAME_WIDTH: usize = 492;
    pub const QUEST_TRACKER_FRAME_HEIGHT: usize = 46;
    pub const QUEST_TRACKER_FRAME_SIZE: usize = QUEST_TRACKER_FRAME_WIDTH * QUEST_TRACKER_FRAME_HEIGHT;
    pub const QUEST_TRACKER_BUFFER_SIZE: usize = QUEST_TRACKER_FRAME_SIZE / 8;
    pub type QuestTrackerFrameData = BitArr!(for QUEST_TRACKER_FRAME_SIZE, in u8, Msb0);

    use deku::prelude::*;
    #[cfg(feature = "wasm")]
    use serde::Serialize;
    #[cfg(feature = "wasm")]
    use tsify_next::Tsify;

    #[repr(transparent)]
    #[derive(Debug, Clone, DekuRead, DekuWrite)]
    #[cfg_attr(feature = "wasm", derive(Serialize, Tsify))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    pub struct QuestTrackerFrame(#[deku(count = "QUEST_TRACKER_BUFFER_SIZE")] Vec<u8>);

    pub trait ToQuestTrackerFrame {
        fn to_quest_tracker_frame(&self) -> QuestTrackerFrame {

        }

        /// Get if the pixel at the given coordinates is set
        /// Return true for a black pixel (1) and false for a white pixel (0)
        fn get_pixel(&self, x: usize, y: usize) -> bool;
    }
}

pub use decl::*;