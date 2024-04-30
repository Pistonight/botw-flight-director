
use std::ffi::CString;
use botwfddata::Payload;
use opencv::prelude::*;

mod server;
use server::Server;
mod log;


/// cbindgen:ignore
#[repr(C)]
pub struct BotwFdCore {
    pub settings: BotwFdSettings,
    /// Id to identify server restarts
    server_id: u32,
    /// Id to identify server restarts
    server: Option<Server>,
}

impl BotwFdCore {
    pub fn new() -> Self {
        BotwFdCore {
            settings: Default::default(),
            server_id: 1,
            server: None,
        }
    }

    pub fn on_setting_update(&mut self) {
        crate::info!("updating settings");
        // Check if server should be started/stopped/restarted
        if self.settings.enable {
            let should_restart = match &self.server {
                Some(server) => {
                    if !server.is_running() || server.get_port() != self.settings.port || server.is_exposed() != self.settings.expose_host {
                        crate::info!("restart server due to settings change");
                        true
                    } else {
                        false
                    }
                }
                None => {
                    true
                }
            };

            if should_restart {
                self.shutdown_server();
                let id = self.server_id;
                self.server_id += 1;
                self.server = Some(Server::start(id, self.settings.expose_host, self.settings.port));
            }

        } else {
            self.shutdown_server();
        }

        crate::info!("settings updated");
    }

    fn shutdown_server(&mut self) {
        if let Some(mut server) = self.server.take() {
            if server.is_running() {
                server.shutdown();
            }
        }
    }

    pub fn process_frame(&self) {
        let server = match &self.server {
            Some(server) => server,
            None => return,
        };
        server.send(Payload::Log(CString::new("Hello from core").unwrap()))
    }

}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BotwFdSettings {
    // Server
    /// The master switch to enable/disable the server
    pub enable: bool,
    /// If the server should be exposed to network
    pub expose_host: bool,
    /// The port the server should listen on
    pub port: u16,
    // Quest Tracker
    pub enable_quest_tracker: bool,
    pub enable_quest_tracker_debug: bool,



    pub enable_direction_guide: bool,
    pub enable_direction_guide_debug: bool,
}

#[no_mangle]
pub extern "C" fn botwfd_load(){
    info!("core loaded");
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
pub extern "C" fn botwfd_get_settings(core: *mut BotwFdCore) -> *mut BotwFdSettings {
    core::ptr::from_mut(&mut unsafe { &mut *core }.settings)
}

#[no_mangle]
pub extern "C" fn botwfd_update_settings(core: *mut BotwFdCore) {
    unsafe { &mut *core }.on_setting_update();
}

#[no_mangle]
pub extern "C" fn botwfd_process_frame(core: *mut BotwFdCore) {
    unsafe { &mut *core }.process_frame();
}

pub fn test() {
    let mat = unsafe { Mat::new_rows_cols(100, 100, opencv::core::CV_8UC1) };
}