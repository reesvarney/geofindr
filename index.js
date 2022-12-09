import express from 'express';
const app  = express();

app.get('/', (req, res)=>{
  res.sendFile('./client/index.html');
});

app.use('/', express.static('./client'));