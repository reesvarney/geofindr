#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::copy, path::Path, sync::Mutex};
use uuid::Uuid;

// Hashmap to store list of all games
lazy_static! {
    static ref GAMES: Mutex<HashMap<String, [Coordinates; 2]>> = Mutex::new(HashMap::new());
}

// Coordinate struct, contains longitude and latitude of a position
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
struct Coordinates {
    lat: f64,
    lng: f64,
}

// Game start response struct, required so that data can be serialised
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GameStartResponse {
    #[serde(rename(serialize = "startPosition"))]
    start_position: Coordinates,
    #[serde(rename(serialize = "gameId"))]
    game_id: String,
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
#[get("/start_game")]
fn start_game() -> Json<GameStartResponse> {
    let id = Uuid::new_v4().to_string();
    // TODO: Get two random positions from list
    // For the first position, get random from list
    // For the second position, get random from list and then check if it is within the min/ max distance, if not - go to next item in list
    // Also make sure it is not the same coordinates as the first position
    let positions = TEST_POSITIONS;
    let mut _games = GAMES.lock().unwrap();
    _games.insert(id.clone(), positions);
    return Json(GameStartResponse {
        start_position: positions[0],
        game_id: id,
    });
}

// Check position distance
#[get("/check_position?<lat>&<lng>&<game_id>")]
fn check_position(lat: f64, lng: f64, game_id: String) -> String {
    let distance: f64 = haversine_distance(Coordinates { lat, lng }, get_coordinates(game_id));
    // TODO: Handle if they have reached the destination on the server side
    return distance.to_string();
}

// Get preview image for game
#[get("/preview?<game_id>")]
async fn preview(game_id: String) -> Option<NamedFile> {
    let coordinates = get_coordinates(game_id);
    let lat = coordinates.lat;
    let lng = coordinates.lng;
    let image_path = format!("../images/{lat}_{lng}.jpeg");
    if !Path::new(&image_path).exists() {
        let url = format!("https://maps.googleapis.com/maps/api/streetview?size=800x400&location={lat},{lng}&fov=120&heading=0&pitch=10&key=AIzaSyCRKDkVX2aluPcBjeEkydAAIf9NQOjmU70");
        download_file(&url, &image_path).await.ok();
    }
    return NamedFile::open(image_path).await.ok();
}

// Gets coordinates
// The lock method is required and stops other parts of the code interacting with the variable
// It only becomes "unlocked" at the end of its scope which is why it needs to be used through a separate function
fn get_coordinates(game_id: String) -> Coordinates {
    let mut _games = GAMES.lock().unwrap();
    let lat = _games[&game_id][1].lat.clone();
    let lng = _games[&game_id][1].lng.clone();
    return Coordinates { lat, lng };
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
        .mount("/", rocket::fs::FileServer::from("../client"))
        .mount("/", routes![start_game, preview, check_position])
}
