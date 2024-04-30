
mod quest_tracker;
pub use quest_tracker::*;

#[allow(non_snake_case)]
mod decl {
    use super::*;
    use std::ffi::CString;
    
    use deku::prelude::*;
    #[cfg(feature = "wasm")]
    use serde::Serialize;
    #[cfg(feature = "wasm")]
    use tsify_next::Tsify;
    
    
    #[derive(Debug, Clone, DekuRead, DekuWrite)]
    #[deku(type = "u8")]
    #[cfg_attr(feature = "wasm", derive(Serialize, Tsify))]
    #[cfg_attr(feature = "wasm", serde(tag = "type", content = "data"))]
    #[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
    pub enum Payload {
        /// A log message
        #[deku(id = "0")]
        Log(CString),

        /// A Quest Tracker frame containing banner data
        #[deku(id = "1")]
        QuestTrackerFrame(quest_tracker::QuestTrackerFrame)
    }

    impl Payload {
        pub fn new_log(msg: &str) -> Self {
            Self::Log(CString::new(msg).unwrap())
        }
    }
}

pub use decl::Payload;