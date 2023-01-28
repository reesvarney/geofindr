let panorama = null;
let game_id = null;

// Runs when the google streetview API is ready
async function initMap() {
  const startPosition = await initGame();
  document.getElementById("preview_image").src = `/preview?game_id=${game_id}`;
  panorama = new google.maps.StreetViewPanorama(
    document.getElementById("sv_container"),
    {
      position: startPosition,
      pov: {
        heading: 0,
        pitch: 10,
      },
    }
  );
  panorama.addListener("position_changed", trackMovement);
}

// Initialises the game with the server
async function initGame() {
  const max_distance = 1000;
  let user_id = "test";
  const data = await (await fetch(`/start_game?user_id=${user_id}`)).json();
  game_id = data.gameId;
  return data.startPosition;
}

// Responds to the change in movement on 
async function trackMovement() {
  const pos = panorama.getPosition().toJSON();
  const distance = await checkDistance(pos);
  document.getElementById("distance_text").innerText = `${Math.floor(distance)}m`;
  if (distance < 20) {
    console.log("Reached end position!");
    // Handle round finish
  }
}

// Retrieves distance from the server
async function checkDistance(position) {
  const distance = Number(await (await fetch(`/check_position?game_id=${game_id}&lat=${position.lat}&lng=${position.lng}`)).text());
  if(distance == NaN){
    throw Error("Undexpected distance value from server")
  }
  return distance
}
