#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, json, Deserialize, Serialize};
use std::{fs::File, io::copy, path::Path};
use uuid::Uuid;
use std::time::{SystemTime};

// Hashmap to store list of all games
type Session<'a> = rocket_session::Session<'a, String>;

// Coordinate struct, contains longitude and latitude of a position
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
struct Coordinates {
    lat: f64,
    lng: f64,
}

#[derive(Clone, Serialize, Deserialize)]
struct GameData {
    coordinates: [Coordinates; 2],
    start_time: SystemTime,
    user_id: String,
}

// Game start response struct, required so that data can be serialised
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GameStartResponse {
    #[serde(rename(serialize = "startPosition"))]
    start_position: Coordinates
}

// TODO: Convert this into a large list of positions
const TEST_POSITIONS: [Coordinates; 2] = [
    Coordinates {
        lat: 50.800022,
        lng: -1.095664,
    },
    Coordinates {
        lat: 50.797585,
        lng: -1.095735,
    },
];

// Initialise game data
#[get("/start_game?<user_id>")]
fn start_game(session: Session, user_id: String) -> Json<GameStartResponse> {
    // TODO: Get two random positions from list
    // For the first position, get random from list
    // For the second position, get random from list and then check if it is within the min/ max distance, if not - go to next item in list
    // Also make sure it is not the same coordinates as the first position
    let positions = TEST_POSITIONS;
    let data = GameData {
        coordinates: positions,
        start_time: SystemTime::now(),
        user_id: user_id,
    };
     session.tap(|n| {
            *n = serde_json::to_string(&data).unwrap();
        });

    return Json(GameStartResponse {
        start_position: positions[0]
    });
}

// Check position distance
#[get("/check_position?<lat>&<lng>")]
async fn check_position(session: Session<'_>, lat: f64, lng: f64) -> String {
    let data: GameData =  session.tap(|n| {
        let x : GameData = serde_json::from_str(&n).unwrap();
        return x;
    });
    let distance: f64 = haversine_distance(Coordinates { lat, lng }, data.coordinates[1]);
    if distance < 20.0 {
        let time_taken = data.start_time.elapsed().ok().expect("Error sending a time result").as_secs();
        let user = data.user_id.to_string();
        let start_pos = json::to_string(&data.coordinates[0]).expect("Error sending a start_pos result");
        let end_pos = json::to_string(&data.coordinates[1]).expect("Error sending an end_pos result");
        let url = format!("https://docs.google.com/forms/d/e/1FAIpQLSdMCNxP4QEmAjuFwQYAZ678P19u08BN0lCvhJffeK4JH5XyYg/formResponse?submit=Submit&usp=pp_url&entry.910322073={user}&entry.1918876988={time_taken}&entry.1744420129={start_pos}&entry.209514963={end_pos}");
        println!("{}", url);
        let resp = reqwest::get(url).await.ok().expect("Error requesting a google forms url");
        if resp.status().is_success() {
            println!("success!");
        } else if resp.status().is_server_error() {
            println!("server error!");
        } else {
            println!("Something else happened. Status: {:?}", resp.status());
        }
    }
    return distance.to_string();
}



// Get preview image for game
#[get("/preview")]
async fn preview(session: Session<'_>) -> Option<NamedFile> {
    let coordinates: Coordinates = session.tap(|n| {
        let data: GameData = serde_json::from_str(&n).unwrap();
        return data.coordinates[1];
    });
    let lat = coordinates.lat;
    let lng = coordinates.lng;
    let image_path = format!("../images/{lat}_{lng}.jpeg");
    if !Path::new(&image_path).exists() {
        let url = format!("https://maps.googleapis.com/maps/api/streetview?size=800x400&location={lat},{lng}&fov=120&heading=0&pitch=10&key=AIzaSyCRKDkVX2aluPcBjeEkydAAIf9NQOjmU70");
        download_file(&url, &image_path).await.ok();
    }
    return NamedFile::open(image_path).await.ok();
}

const MULT: f64 = 180.0 / std::f64::consts::PI;
    
// Calculates distance between 2 coordinates 
fn haversine_distance(pos1: Coordinates, pos2: Coordinates) -> f64 {
    return ((pos1.lat / MULT).sin() * (pos2.lat / MULT).sin()
        + (pos1.lat / MULT).cos()
            * (pos2.lat / MULT).cos()
            * (pos2.lng / MULT - pos1.lng / MULT).cos())
    .acos()
        * 6371000.0
}

// Downloads a file from a URL
async fn download_file(url: &String, location: &String) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut dest = { File::create(location)? };
    let mut content = std::io::Cursor::new(response.bytes().await?);
    copy(&mut content, &mut dest)?;
    return Ok(());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Session::fairing())
        .mount("/", rocket::fs::FileServer::from("../client"))
        .mount("/", routes![start_game, preview, check_position])
}
