import express from 'express';
import url from 'url';
import fetch from 'node-fetch';
import fs from 'fs';
import { randomUUID } from 'crypto';

// Serve main html file
const app  = express();
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
app.get('/', (req, res)=>{
  res.sendFile('./client/index.html', {root: __dirname});
}); 
app.use('/', express.static('./client'));

// TODO: Create list of random positions
// Also possibly include a text address/ description of the location
const testPositions = {
  start: {lat: 50.800022, lng: -1.095664},
  finish: {lat: 50.797585, lng: -1.095735},
};

const games = {};

// USAGE: /start_game?max_distance=[DISTANCE IN METERS]
// Returns starting position & game ID, stores finish position
app.get('/start_game', (req, res)=>{
  const maxDistance = req.query.max_distance;
  const area = "portsmouth";
  const game_id = randomUUID();
  // TODO: Retrieve random positions from list
  const positions = testPositions;
  games[game_id] = positions;
  res.json({
    startPosition: positions.start,
    gameId: game_id
  })
});

// USAGE: /preview?game_id=[GAME ID]
// Returns static image of finish position
app.get('/preview', async(req, res)=>{
  const pos = games[req.query.game_id].finish;
  const imagePath=`./images/${pos.lat}_${pos.lng}.jpeg`;
  if (!fs.existsSync(imagePath)) {
    const image = await fetch(`https://maps.googleapis.com/maps/api/streetview?size=800x400&location=${pos.lat},${pos.lng}&fov=120&heading=0&pitch=10&key=AIzaSyCRKDkVX2aluPcBjeEkydAAIf9NQOjmU70`)
    const fileStream = fs.createWriteStream(imagePath);
    image.body.pipe(fileStream);
    fileStream.on("finish", ()=>{
      res.sendFile(imagePath, {root: __dirname});
    });
    return;
  }
  res.sendFile(imagePath, {root: __dirname});
})

// USAGE: /check_position?game_id=[GAME ID]&lat=[LATITUDE]&lng=[LONGITUDE]
// Returns distance to finish position in meters
app.get('/check_position', (req, res)=>{
  const {lat, lng, game_id} = req.query;
  const distance = haversineDistance({lat, lng}, games[game_id].finish);
  res.send(distance.toString());
})

/**
 * Calculates distance in meters between two lat/long coordinates using the haversine formula
 * @param {{lat: number, lng: number}} pos1 First position
 * @param {{lat: number, lng: number}} pos2 Second Position
 * @returns {number} Distance in meters
 */
function haversineDistance(pos1, pos2){
  const mult = 180 / Math.PI;
  return Math.acos(
    Math.sin(pos1.lat/ mult) * Math.sin(pos2.lat/ mult)
    + Math.cos(pos1.lat / mult) * Math.cos(pos2.lat/ mult) * Math.cos(pos2.lng/mult -pos1.lng / mult)
    ) * 6371 * 1000
};

app.listen(80, () => {
  console.log(`Example app listening on port ${80}`)
})