#[macro_use]
extern crate rocket;
extern crate rand;

//use rocket::http::hyper::uri::Port;
use rand::Rng;
use rocket::fs::NamedFile;
use rocket::serde::{json, json::Json, Deserialize, Serialize};
use std::time::SystemTime;
use std::{fs::File, io::copy, path::Path};

const FORM_ID: &str = ""; // Google Form ID
const API_KEY: &str = ""; // Google API Key

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
    start_position: Coordinates,
}

// TODO: Convert this into a large list of positions
const PORT_COORDS: [Coordinates; 50] = [
    Coordinates {
        lat: 50.800022,
        lng: -1.095664,
    },
    Coordinates {
        lat: 50.799807,
        lng: -1.095634,
    },
    Coordinates {
        lat: 50.797585,
        lng: -1.095735,
    },
    Coordinates {
        lat: 50.795546,
        lng: -1.091959,
    },
    Coordinates {
        lat: 50.788019,
        lng: -1.055693,
    },
    Coordinates {
        lat: 50.809171,
        lng: -1.086770,
    },
    Coordinates {
        lat: 50.823145,
        lng: -1.060117,
    },
    Coordinates {
        lat: 50.781742,
        lng: -1.086038,
    },
    Coordinates {
        lat: 50.784100,
        lng: -1.078329,
    },
    Coordinates {
        lat: 50.789016,
        lng: -1.085832,
    },
    Coordinates {
        lat: 50.794960,
        lng: -1.067656,
    },
    Coordinates {
        lat: 50.80377248,
        lng: -1.082880862,
    },
    Coordinates {
        lat: 50.79993,
        lng: -1.08079,
    },
    Coordinates {
        lat: 50.80548,
        lng: -1.07683,
    },
    Coordinates {
        lat: 50.80319,
        lng: -1.09058,
    },
    Coordinates {
        lat: 50.79508,
        lng: -1.07949,
    },
    Coordinates {
        lat: 50.8026,
        lng: -1.06366,
    },
    Coordinates {
        lat: 50.79444,
        lng: -1.0651,
    },
    Coordinates {
        lat: 50.79096,
        lng: -1.07628,
    },
    Coordinates {
        lat: 50.79512,
        lng: -1.08698,
    },
    Coordinates {
        lat: 50.79142,
        lng: -1.09571,
    },
    Coordinates {
        lat: 50.79073,
        lng: -1.09127,
    },
    Coordinates {
        lat: 50.78934,
        lng: -1.08264,
    },
    Coordinates {
        lat: 50.79135,
        lng: -1.08222,
    },
    Coordinates {
        lat: 50.79332,
        lng: -1.08548,
    },
    Coordinates {
        lat: 50.7947,
        lng: -1.09598,
    },
    Coordinates {
        lat: 50.80112,
        lng: -1.07327,
    },
    Coordinates {
        lat: 50.79693,
        lng: -1.07682,
    },
    Coordinates {
        lat: 50.80252,
        lng: -1.07729,
    },
    Coordinates {
        lat: 50.78648,
        lng: -1.07223,
    },
    Coordinates {
        lat: 50.78677,
        lng: -1.07844,
    },
    Coordinates {
        lat: 50.78829,
        lng: -1.07655,
    },
    Coordinates {
        lat: 50.78437,
        lng: -1.0845,
    },
    Coordinates {
        lat: 50.7837,
        lng: -1.092,
    },
    Coordinates {
        lat: 50.78568,
        lng: -1.09471,
    },
    Coordinates {
        lat: 50.78782,
        lng: -1.09829,
    },
    Coordinates {
        lat: 50.79269,
        lng: -1.0996,
    },
    Coordinates {
        lat: 50.79303,
        lng: -1.10456,
    },
    Coordinates {
        lat: 50.79448,
        lng: -1.10299,
    },
    Coordinates {
        lat: 50.79,
        lng: -1.10446,
    },
    Coordinates {
        lat: 50.79101,
        lng: -1.10208,
    },
    Coordinates {
        lat: 50.78991,
        lng: -1.09601,
    },
    Coordinates {
        lat: 50.78671,
        lng: -1.08905,
    },
    Coordinates {
        lat: 50.78931,
        lng: -1.09313,
    },
    Coordinates {
        lat: 50.78129,
        lng: -1.0803,
    },
    Coordinates {
        lat: 50.7801,
        lng: -1.08408,
    },
    Coordinates {
        lat: 50.79253,
        lng: -1.08847,
    },
    Coordinates {
        lat: 50.79388,
        lng: -1.09146,
    },
    Coordinates {
        lat: 50.79345,
        lng: -1.09596,
    },
    Coordinates {
        lat: 50.81705,
        lng: -1.07946,
    },
];

fn get_random_position(positions: &[Coordinates]) -> Coordinates {
    let random_index = rand::thread_rng().gen_range(0, positions.len() - 1);
    positions[random_index]
}

// Initialise game data
#[get("/start_game?<user_id>&<min_dist>&<max_dist>")]
fn start_game(
    session: Session,
    user_id: String,
    min_dist: f64,
    max_dist: f64,
) -> Json<GameStartResponse> {
    let first_pos = get_random_position(&PORT_COORDS);
    let mut second_pos = get_random_position(&PORT_COORDS);

    // Ensure second position is not the same as the first
    let mut index = rand::thread_rng().gen_range(0, PORT_COORDS.len());
    let mut counter = 0;
    while counter < PORT_COORDS.len() {
        let pos = PORT_COORDS[index];
        let distance = haversine_distance(pos, first_pos);
        if (distance > min_dist && distance < max_dist)
            && (pos.lat != first_pos.lat && pos.lng != first_pos.lng)
        {
            second_pos = pos;
            break;
        }
        if index < PORT_COORDS.len() - 1 {
            index += 1;
        } else if index == PORT_COORDS.len() - 1 {
            index = 0;
        }
        counter += 1;
    }
    let positions = [first_pos, second_pos];
    let data = GameData {
        coordinates: positions,
        start_time: SystemTime::now(),
        user_id: user_id,
    };
    session.tap(|n| {
        *n = serde_json::to_string(&data).unwrap();
    });

    return Json(GameStartResponse {
        start_position: first_pos,
    });
}

// Check position distance
#[get("/check_position?<lat>&<lng>")]
async fn check_position(session: Session<'_>, lat: f64, lng: f64) -> String {
    let data: GameData = session.tap(|n| {
        let x: GameData = serde_json::from_str(&n).unwrap();
        return x;
    });
    let distance: f64 = haversine_distance(Coordinates { lat, lng }, data.coordinates[1]);
    if distance < 40.0 {
        let time_taken = data
            .start_time
            .elapsed()
            .ok()
            .expect("Error sending a time result")
            .as_secs();
        let user = data.user_id.to_string();
        let start_pos =
            json::to_string(&data.coordinates[0]).expect("Error sending a start_pos result");
        let end_pos =
            json::to_string(&data.coordinates[1]).expect("Error sending an end_pos result");
        // Send result to google forms for data analysis
        let url = format!("https://docs.google.com/forms/d/e/{FORM_ID}/formResponse?submit=Submit&usp=pp_url&entry.910322073={user}&entry.1918876988={time_taken}&entry.1744420129={start_pos}&entry.209514963={end_pos}");
        let resp = reqwest::get(url)
            .await
            .ok()
            .expect("Error requesting a google forms url");
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
        let url = format!("https://maps.googleapis.com/maps/api/streetview?size=800x400&location={lat},{lng}&fov=120&heading=0&pitch=10&key={API_KEY}");
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
        * 6371000.0;
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
    println!("Starting Server");
    rocket::build()
        .attach(Session::fairing())
        .mount("/", rocket::fs::FileServer::from("../client"))
        .mount("/", routes![start_game, preview, check_position])
}
