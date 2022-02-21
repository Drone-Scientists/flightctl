use std::error::Error;
const MAVSDK_PROTO_PATH: &str = "MAVSDK-Proto/protos";

const APIS: &[&str] = &["info", "telemetry"];

fn main() -> Result<(), Box<dyn Error>> {
    for api in APIS {
        let api_path = format!("{0}/{1}/{1}.proto", MAVSDK_PROTO_PATH, api);
        tonic_build::configure()
            .build_server(false)
            .compile(&[api_path.as_str()], &[MAVSDK_PROTO_PATH])?;
    }
    Ok(())
}
