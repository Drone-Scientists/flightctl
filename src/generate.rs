use serde_json::{json, Value};
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

// LineMission uses 3 drones to create a line shape with width (m) and angle (rad)
pub struct LineMission {
    // width of line
    width: u8,
    // angle of line in radians relative to the earth's latitude
    angle: f64,
    // start indicates the location the drone starts at
    // altitude is ignored
    start: Point,
    // target_location specifies the center point where the circle would be generated
    target_location: Point,
    // number of sec to hold the shape
    hold_sec: u8,
}

impl LineMission {
    pub fn new(
        width: u8,
        angle: f64,
        start_lat: f64,
        start_lon: f64,
        target_lat: f64,
        target_lon: f64,
        target_alt: u8,
        hold_sec: u8,
    ) -> LineMission {
        LineMission {
            width,
            angle,
            start: Point::new(start_lat, start_lon, 0, 0),
            target_location: Point::new(target_lat, target_lon, target_alt, 0),
            hold_sec,
        }
    }
}

impl ShapeMission for LineMission {
    fn generate_missions(&self) -> Vec<Vec<Point>> {
        let mut ret = vec![];
        let dy = (self.width / 2) as f64 * self.angle.sin();
        let dx = (self.width / 2) as f64 * self.angle.cos();
        for i in vec![1_f64, -1_f64] {
            // 1 degree lat = 111111 m
            let dlat_degree = dy * i / 111111_f64;
            // 1 degree lon = 111111 * cos(lat) m
            let dlon_degree = dx * i / (111111_f64 * (self.target_location.lon * TO_RAD).cos());

            let mut mission = vec![];
            mission.push(self.start.clone());
            mission.push(self.target_location.clone());
            mission.push(Point::new(
                self.target_location.lat + dlat_degree,
                self.target_location.lon + dlon_degree,
                self.target_location.alt,
                self.hold_sec,
            ));
            ret.push(mission);
        }
        // push middle drone mission
        ret.push(vec![
            self.start.clone(),
            self.target_location.clone(),
            Point::new(
                self.target_location.lat,
                self.target_location.lon,
                self.target_location.alt,
                self.hold_sec,
            ),
        ]);
        ret
    }
}

// SquareMission uses 4 drones to create a square shape with width (m)
pub struct SquareMission {
    // width of a square side
    width: u8,
    // start indicates the location the drone starts at
    // altitude is ignored
    start: Point,
    // target_location specifies the center point where the circle would be generated
    target_location: Point,
    // number of sec to hold the shape
    hold_sec: u8,
}

impl SquareMission {
    pub fn new(
        width: u8,
        start_lat: f64,
        start_lon: f64,
        target_lat: f64,
        target_lon: f64,
        target_alt: u8,
        hold_sec: u8,
    ) -> SquareMission {
        SquareMission {
            width,
            start: Point::new(start_lat, start_lon, 0, 0),
            target_location: Point::new(target_lat, target_lon, target_alt, 0),
            hold_sec,
        }
    }
}

impl ShapeMission for SquareMission {
    fn generate_missions(&self) -> Vec<Vec<Point>> {
        let mut ret = vec![];
        for (mut dx, mut dy) in vec![(0.5, 0.5), (-0.5, 0.5), (-0.5, -0.5), (0.5, -0.5)] {
            dy *= self.width as f64;
            dx *= self.width as f64;

            // 1 degree lat = 111111 m
            let dlat_degree = dy / 111111_f64;
            // 1 degree lon = 111111 * cos(lat) m
            let dlon_degree = dx / (111111_f64 * (self.target_location.lon * TO_RAD).cos());

            let mut mission = vec![];
            mission.push(self.start.clone());
            mission.push(self.target_location.clone());
            mission.push(Point::new(
                self.target_location.lat + dlat_degree,
                self.target_location.lon + dlon_degree,
                self.target_location.alt,
                self.hold_sec,
            ));
            ret.push(mission);
        }
        ret
    }
}

static TO_RAD: f64 = PI / 180_f64;

// CircleMission uses n drones (count) to create a circle shape with radius (m)
pub struct CircleMission {
    // radius is in meters
    radius: u8,
    count: u8,
    // start indicates the location the drone starts at
    // altitude is ignored
    start: Point,
    // target_location specifies the center point where the circle would be generated
    target_location: Point,
    // number of sec to hold the shape
    hold_sec: u8,
}

impl CircleMission {
    pub fn new(
        count: u8,
        radius: u8,
        start_lat: f64,
        start_lon: f64,
        target_lat: f64,
        target_lon: f64,
        target_alt: u8,
        hold_sec: u8,
    ) -> CircleMission {
        CircleMission {
            radius,
            count,
            start: Point::new(start_lat, start_lon, 0, 0),
            target_location: Point::new(target_lat, target_lon, target_alt, 0),
            hold_sec,
        }
    }
}

impl ShapeMission for CircleMission {
    fn generate_missions(&self) -> Vec<Vec<Point>> {
        let mut ret = vec![];

        let segment = 2.0 * PI / (self.count as f64);
        for i in 0..self.count {
            let segment_rad = segment * (i as f64);
            let dy_m = (self.radius as f64) * segment_rad.sin();
            let dx_m = (self.radius as f64) * segment_rad.cos();

            // 1 degree lat = 111111 m
            let dlat_degree = dy_m / 111111_f64;
            // 1 degree lon = 111111 * cos(lat) m
            let dlon_degree = dx_m / (111111_f64 * (self.target_location.lon * TO_RAD).cos());

            let mut mission = vec![];
            mission.push(self.start.clone());
            mission.push(self.target_location.clone());
            mission.push(Point::new(
                self.target_location.lat + dlat_degree,
                self.target_location.lon + dlon_degree,
                self.target_location.alt,
                self.hold_sec,
            ));
            ret.push(mission);
        }

        ret
    }
}

struct Point {
    lat: f64,
    lon: f64,
    alt: u8,
    hold_sec: u8,
}

impl Point {
    fn new(lat: f64, lon: f64, alt: u8, hold_sec: u8) -> Point {
        Point {
            lat,
            lon,
            alt,
            hold_sec,
        }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point::new(self.lat, self.lon, self.alt, self.hold_sec)
    }
}

pub trait ShapeMission {
    // generate missions creates a series missions made a list of of lat, lon coordinate tuples
    // the first tuple must be the starting location
    fn generate_missions(&self) -> Vec<Vec<Point>>;
    fn write_mission_to_disk(&self, save_dir: &Path) -> io::Result<()> {
        if !save_dir.is_dir() {
            io::Error::new(io::ErrorKind::Other, "Not a directory");
        }
        let mut i = 0;
        for mission in self.generate_missions() {
            let plan_path = save_dir.join(format!("plan_{}.plan", i));
            let mut file = File::create(plan_path)?;

            println!("Writing plan {} to file {}", i, plan_path.display());
            file.write_all(self.generate_plan(mission).as_bytes())?;
            i += 1;
        }
        Ok(())
    }
    fn generate_plan(&self, waypoints: Vec<Point>) -> String {
        let mut plan = new_qgc_plan();

        for i in 0..(waypoints.len() + 1) {
            if i == 0 {
                plan["mission"]["items"][i] = new_takeoff(waypoints[i].clone());
            } else if i == waypoints.len() {
                plan["mission"]["items"][i] = new_return((i + 1) as u8)
            } else {
                plan["mission"]["items"][i] = new_waypoint(waypoints[i].clone(), (i + 1) as u8)
            }
        }

        plan.to_string()
    }
}

fn new_takeoff(initial_position: Point) -> Value {
    json!({
        "AMSLAltAboveTerrain": null,
        "Altitude": 50,
        "AltitudeMode": 1,
        "autoContinue": true,
        "command": 22,
                "doJumpId": 1,
                "frame": 3,
                "params": [
                    0,
                    0,
                    0,
                    null,
                    initial_position.lat,
                    initial_position.lon,
                    initial_position.alt,
                ],
                "type": "SimpleItem"
    })
}

fn new_waypoint(location: Point, jump_id: u8) -> Value {
    json!({
        "AMSLAltAboveTerrain": null,
        "Altitude": 50,
        "AltitudeMode": 1,
        "autoContinue": true,
        "command": 16,
        "doJumpId": jump_id,
        "frame": 3,
        "params": [
            location.hold_sec,
            0,
            0,
            null,
            location.lat,
            location.lon,
            location.alt,
        ],
        "type": "SimpleItem"
    })
}

fn new_return(jump_id: u8) -> Value {
    json!({
        "autoContinue": true,
        "command": 20,
        "doJumpId": jump_id,
        "frame": 2,
        "params": [
            0,
            0,
            0,
            0,
            0,
            0,
            0
        ],
        "type": "SimpleItem"
    })
}

fn new_qgc_plan() -> Value {
    json!({
        "fileType": "Plan",
        "geoFence": {
            "circles": [],
            "polygons": [],
            "version": 2
        },
        "groundStation": "QGroundControl",
        "mission": {
            "cruiseSpeed": 15,
            "firmwareType": 12,
            "globalPlanAltitudeMode": 1,
            "hoverSpeed": 5,
            "items": [],
            "plannedHomePosition": [
                0,
                0,
                50
            ],
            "vehicleType": 2,
            "version": 2
        },
        "rallyPoints": {
            "points": [],
            "version": 2
        },
        "version": 1
    })
}
