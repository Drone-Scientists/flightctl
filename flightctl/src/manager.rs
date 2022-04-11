use mavsdk::ffi;
use mavsdk::ffi::{new_fmavsdk, FMavsdk};
use std::pin::Pin;
use tokio::task::JoinHandle;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Manager {
    mavsdk: cxx::UniquePtr<FMavsdk>, // Use a raw pointer to point to the opaque FMavsdk
    targets: Vec<Target>,
}

// General Functions Block
impl Manager {
    // new constructs a new manager
    pub fn new() -> Self {
        let t: Vec<_> = Vec::new();
        Self {
            mavsdk: new_fmavsdk(),
            targets: t,
        }
    }

    // add_target initializes and starts a target based on target_url
    pub async fn add_target(&mut self, target_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        
        let t: Target = Target {
            url: target_url.to_string(),
            system: self.mavsdk.pin_mut().connect(target_url.to_string()),
        };
        self.targets.push(t);
        Ok(())
    }

    // add_targets async initializes and starts multiple targets based on their target_urls
    pub async fn add_targets(
        &mut self,
        target_urls: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut handles: Vec<_> = Vec::new();
        // spawn all tasks ahead of time so I/O bound operations don't block
        for t in target_urls {
            let handle: JoinHandle<Option<Target>> = tokio::spawn(async {
                match System::connect(t.clone()).await {
                    Ok(s) => Some(Target { url: t, system: s }),
                    Err(e) => {
                        eprintln!("Connection failed {:?}", e);
                        None
                    }
                }
            });
            handles.push(handle);
        }
        // collect all connected targets
        for h in handles {
            match h.await? {
                Some(t) => self.targets.push(t),
                None => {}
            }
        }
        Ok(())
    }
}

// Info Plugin
impl Manager {
    // get_info connects to all the target drones and fetches info
    // these functions use tokio async tasks (green threads) to fan out connections
    pub async fn get_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut handles: Vec<_> = Vec::new();
        // spawn all tasks ahead of time so I/O bound operations don't block
        for mut t in self.targets.clone() {
            let handle: JoinHandle<_> = tokio::spawn(async move { t.get_info().await });
            handles.push(handle);
        }
        // collect all connected targets
        for h in handles {
            match h.await? {
                Ok(v) => println!("Recieved Version: {:?}", v),
                Err(e) => eprint!("Error getting version: {:?}", e),
            }
        }
        return Ok(());
    }
}

// TODO: Telemetry Plugin
impl Manager {}

#[derive(Clone)]
pub struct Target {
    url: String,
    system: System,
}

impl Target {
    pub async fn get_info(&mut self) -> mavsdk::info::GetVersionResult {
        self.system.info.get_version().await
    }
}
