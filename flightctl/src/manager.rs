use futures::future::join_all;
use mavsdk::System;
use tokio::task::JoinHandle;

pub struct Manager {
    targets: Vec<Target>,
}

impl Manager {
    // new constructs a new manager
    pub fn new() -> Self {
        let mut t: Vec<_> = Vec::new();
        Self { targets: t }
    }

    // get_stats connects to all the target drones and fetches info
    // these functions use tokio async tasks (green threads) to fan out connections
    pub async fn get_stats(self) -> Result<(), Box<dyn std::error::Error>> {
        let handles: Vec<JoinHandle<_>> = Vec::new();
        for t in &self.targets {
            t.get_stats();
            println!("target: {}", t.url);
        }

        // cleanup and exit
        let handles: Vec<JoinHandle<()>> =
            self.targets.into_iter().filter_map(|t| t.handle).collect();
        join_all(handles).await;
        Ok(())
    }

    // add_target initializes and starts a target based on target_url
    pub async fn add_target(&mut self, target_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let t: Target = Target {
            url: target_url.to_string(),
            system: mavsdk::System::connect(target_url.to_string()).await?
        };
        self.targets.push(t);
        Ok(())
    }

    // add_targets async initializes and starts multiple targets based on their target_urls
    pub async fn add_targets(&mut self, target_urls: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut handles: Vec<_> = Vec::new();
        // spawn all tasks ahead of time so I/O bound operations don't block
        for t in target_urls {
            let handle: JoinHandle<Option<Target>> = tokio::spawn(async {
                match System::connect(t.clone()).await {
                    Ok(s) => Some(
                        Target {
                            url: t,
                            system: s
                        }
                    ),
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

pub struct Target {
    url: String,
    system: System,
}

impl Target {
    pub async fn get_stats(&mut self) {
        match self.handle {
            // if the target is already running, don't start another
            Some(_) => {}
            None => {
                let jh: JoinHandle<Result<System, tonic::transport::Error>> =
                    tokio::spawn(async {
                        mavsdk::System::connect(self.url).await?;
                    })
                // convert jh to Option<JoinHandle<()>> type
                self.handle = Some(jh)
            }
        }
    }
}
