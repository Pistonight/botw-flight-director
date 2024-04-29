use std::os::raw;


use std::ffi::CString;

use opencv::prelude::*;

mod server;

// extern "C" {
//     fn botwfdplugin_log(data: *const raw::c_char);
// }

// /// Log a message to OBS
// pub fn botwfd_log(message: &str) {
//     let c_message = CString::new(message).unwrap();
//     unsafe {
//         botwfdplugin_log(c_message.as_ptr());
//     }
// }

#[repr(C)]
pub struct BotwFdCore {
    pub settings: BotwFdSettings,
    //pub server: Option<Server>,
}

impl BotwFdCore {
    pub fn new() -> Self {
        BotwFdCore {
            settings: Default::default(),
            //server: None,
        }
    }

    pub fn on_setting_update(&mut self) {
        // Check if server should be started/stopped/restarted
        // if self.settings.enable {
        //     match &mut self.server {
        //         Some(server) => {
        //             if server.port != self.settings.port {
        //                 // restart server due to port change
        //                 server.shutdown();
        //                 *server = Server::new(self.settings.port);
        //             }
        //         }
        //         None => {
        //             self.server = Some(Server::new(self.settings.port));
        //         }
        //     }

        // } else {
        //     if let Some(mut server) = self.server.take() {
        //         server.shutdown();
        //     }
        // }
    }

}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BotwFdSettings {
    // Server
    pub enable: bool,
    pub enable_logging: bool,
    pub port: u16,
    // Quest Tracker
    pub enable_quest_tracker: bool,
    pub enable_quest_tracker_debug: bool,



    pub enable_direction_guide: bool,
    pub enable_direction_guide_debug: bool,
}

#[no_mangle]
pub extern "C" fn botwfd_load(){
    
}

#[no_mangle]
pub extern "C" fn botwfd_create() -> *mut BotwFdCore {
    Box::into_raw(Box::new(BotwFdCore::new()))
}

#[no_mangle]
pub extern "C" fn botwfd_destroy(core: *mut BotwFdCore) {
    unsafe {
        let _ = Box::from_raw(core);
    }
}

#[no_mangle]
pub extern "C" fn botwfd_update_settings(core: *mut BotwFdCore) {
    unsafe {
        (*core).on_setting_update();
    }
}

pub fn test() {
    let mat = unsafe { Mat::new_rows_cols(100, 100, opencv::core::CV_8UC1) };
}