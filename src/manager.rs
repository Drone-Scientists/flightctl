use std::pin;
use std::sync::Arc;

use cxx::{SharedPtr, UniquePtr};
use tokio::task::JoinHandle;

// #[cfg(not(test))]
use mavsdk::ffi as sdk;
// #[cfg(test)]
// use mock::sdk;
//
// #[cfg(test)]
// mod mock {
//     use mockall::automock;
//     #[automock()]
//     pub(super) mod sdk {
//         use cxx::{SharedPtr, UniquePtr};
//         use std::pin::Pin;
//
//         pub type Mavsdk = mavsdk::ffi::Mavsdk;
//         pub type System = mavsdk::ffi::System;
//         // MAVSDK Plugin Types
//         pub type Telemetry = mavsdk::ffi::Telemetry;
//
//         // Shim Constructors
//         pub fn new_mavsdk() -> UniquePtr<Mavsdk> {
//             UniquePtr::null()
//         }
//
//         pub fn new_telemetry(sys: SharedPtr<System>) -> UniquePtr<Telemetry> {
//             UniquePtr::null()
//         }
//
//         // Helper Functions
//         pub fn connect<'a>(sdk: Pin<&'a mut Mavsdk>, addr: String) -> SharedPtr<System> {
//             SharedPtr::null()
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // white-box test providing manager object creation coverage
    fn manager_constructor() {
        let mut mgr = Manager::new();
        assert!(mgr.targets.is_empty());
        assert!(!(mgr.mavsdk.is_null()))
    }

    #[tokio::test]
    async fn manager_add_target() {
        let mut mgr = Manager::new();
        mgr.add_target("somewhere").await;

        let t = mgr.targets.get(0).unwrap();
        assert_eq!(t.t, "somewhere");
        assert!(!(t.system.is_null()));
    }
}

pub struct Manager {
    mavsdk: UniquePtr<sdk::Mavsdk>, // Use a raw pointer to point to the opaque FMavsdk
    targets: Vec<Target>,
}

// General Functions Block
impl Manager {
    // new constructs a new manager
    pub fn new() -> Self {
        let t: Vec<_> = Vec::new();
        Self {
            mavsdk: unsafe { sdk::new_mavsdk() },
            targets: t,
        }
    }

    // add_target initializes and starts a target based on target_url
    pub async fn add_target(&mut self, target_url: &str) {
        let t: Target = Target {
            t: target_url.to_string(),
            system: unsafe { sdk::connect(self.mavsdk.pin_mut(), target_url.to_string()) },
        };
        self.targets.push(t);
    }

    // add_targets async initializes and starts multiple targets based on their target_urls
    // pub async fn add_targets(
    //     &mut self,
    //     target_urls: Vec<String>,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut handles: Vec<_> = Vec::new();
    //     let mut sdk = Arc::new(self.mavsdk);
    //     // spawn all tasks ahead of time so I/O bound operations don't block
    //     for t in target_urls {
    //         let handle: JoinHandle<Option<Target>> = tokio::spawn(self.add_target(t.as_str()));
    //         handles.push(handle);
    //     }
    //     // collect all connected targets
    //     for h in handles {
    //         match h.await? {
    //             Some(t) => self.targets.push(t),
    //             None => {}
    //         }
    //     }
    //     Ok(())
    // }
}

// Info Plugin
impl Manager {
    // get_info connects to all the target drones and fetches info
    // these functions use tokio async tasks (green threads) to fan out connections
    // pub async fn get_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut handles: Vec<_> = Vec::new();
    //     // spawn all tasks ahead of time so I/O bound operations don't block
    //     for mut t in self.targets.clone() {
    //         let handle: JoinHandle<_> = tokio::spawn(async move { t.get_info().await });
    //         handles.push(handle);
    //     }
    //     // collect all connected targets
    //     for h in handles {
    //         match h.await? {
    //             Ok(v) => println!("Recieved Version: {:?}", v),
    //             Err(e) => eprint!("Error getting version: {:?}", e),
    //         }
    //     }
    //     return Ok(());
    // }
}

// TODO: Telemetry Plugin
impl Manager {}

#[derive(Clone)]
pub struct Target {
    t: String,
    system: SharedPtr<sdk::System>,
}

unsafe impl Send for Target {}

impl Target {
    // pub async fn get_info(&mut self) -> mavsdk::info::GetVersionResult {
    //     self.system.info.get_version().await
    // }
}
