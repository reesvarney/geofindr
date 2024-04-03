let panorama = null;
let game_id = null;
let initDistance = null;
let timeTaken = 0;


// Runs when the google streetview API is ready
async function setMap(startPosition) {
  document.getElementById(
    "preview_img"
  ).src = `/preview?game_id=${game_id}`;
  panorama = new google.maps.StreetViewPanorama(
    document.getElementById("sv_container"),
    {
      position: startPosition,
      showRoadLabels: false,
      pov: {
        heading: 0,
        pitch: 10,
      },
      fullscreenControl: false,
    }
  );
  panorama.addListener("position_changed", trackMovement);
}

// Start button function
async function initMap() {
  if (localStorage.getItem("email")) {
    const el = document.getElementById("nameInput");
    el.value = localStorage.getItem("email");
    el.disabled = "true";
    document.querySelector("#startForm .info.register-only").style.display = "none";
  }
  if(localStorage.getItem("consented")){
    document.getElementById("consent").checked = true;
  }
  document.getElementById("startForm").addEventListener("submit", initGame);
}

// Initialises the game with the server
async function initGame(e) {
  e.preventDefault();
  const email = document.getElementById("nameInput").value;
  localStorage.setItem("consented", true);
  localStorage.setItem("email", email);
  const buffer = new TextEncoder("utf-8").encode(email);
  const hash_bytes = await crypto.subtle.digest("SHA-1", buffer);
  const userID = [...new Uint8Array(hash_bytes)]
    .map((x) => x.toString(16))
    .join("");
  const min_dist = document.getElementById("min_dist").value;
  const max_dist = document.getElementById("max_dist").value;
  const data = await (await fetch(`/start_game?user_id=${userID}&min_dist=${min_dist}&max_dist=${max_dist}`)).json();
  game_id = data.gameId;
  console.log(data);
  setMap(data.startPosition);
  document.getElementById("menuBG").remove();
  timer();
}

// Responds to the change in movement on
async function trackMovement() {
  const pos = panorama.getPosition().toJSON();
  const distance = await checkDistance(pos);
  if(initDistance == null) {
    initDistance = distance;
  }
  document.getElementById("distance_text").innerText = `${Math.floor(
    distance
  )}m`;
  if (distance < 40) {
    console.log("Reached end position!");
    window.clearInterval(interval);
    // Handle round finish
    
    const score = Math.floor(initDistance / timeTaken * 1000);
    document.getElementById("end_distance").innerText = `${Math.floor(initDistance)} Metres`;
    document.getElementById("end_time").innerText = `${timeTaken / 10} seconds`;
    document.getElementById("end_score").innerText = Math.floor(score);
    document.getElementById("endScreen").style.visibility = "visible";
    let hs = localStorage.getItem("highScore")|| 0;
    console.log(localStorage.getItem("highScore"), hs);
    if(hs < score) {
      localStorage.setItem("highScore", score);
      document.getElementById("newHS").style.visibility = "visible";
    }
    document.getElementById("end_highScore").innerText = hs;

  }
}

// Retrieves distance from the server
async function checkDistance(position) {
  const distance = Number(
    await (
      await fetch(
        `/check_position?game_id=${game_id}&lat=${position.lat}&lng=${position.lng}`
      )
    ).text()
  );
  if (distance == NaN) {
    throw Error("Unexpected distance value from server");
  }
  return distance;
}

let interval = null;

function timer() { 
  interval = setInterval(function(){
    timeTaken++;

  document.getElementById("timer").textContent = `${Math.floor(timeTaken / 600).toString().padStart(2, "0")}:${(timeTaken / 10 % 60).toFixed(1).padStart(4, "0")}`;
  }, 100);
}
