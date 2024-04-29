use std::ffi::CString;

use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum Payload {
    #[deku(id = "0")]
    Log(CString),
}