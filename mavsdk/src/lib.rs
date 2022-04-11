#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[cxx::bridge(namespace = "mavsdk")]
pub mod ffi {

    pub struct FMissionItem {
        lat_deg: f64,
        lon_deg: f64,
        rel_alt_m: f32,
        speed_m_s: f32,
        is_fly_through: bool,
    }

    unsafe extern "C++" {
        include!("mavsdk/include/shim.h");
        include!("mavsdk/include/helper.h");

        // MAVSDK Core Types
        pub type Mavsdk;
        pub type System;
        // MAVSDK Plugin Types
        pub type Telemetry;

        // Custom Types

        // Shim Constructors
        pub fn new_mavsdk() -> UniquePtr<Mavsdk>;
        pub fn new_telemetry(sys: SharedPtr<System>) -> UniquePtr<Telemetry>;

        // Function Mappings
        pub fn health_all_ok(self: &Telemetry) -> bool;

        // Helper Functions
        pub fn connect(sdk: Pin<&mut Mavsdk>, addr: String) -> SharedPtr<System>;
    }
}
