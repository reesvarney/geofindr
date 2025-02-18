GeoFindr 🚑🌍

GeoFindr is a gamified, GeoGuessr-style web application designed specifically for ambulance drivers in the Portsmouth area. The goal of this project was to enhance geographic familiarity and knowledge of the area in order to improve response efficiency by providing an interactive and engaging way for drivers to recognize key locations in order to quickly and effectively arrive to them.


🎯 Purpose

Emergency response teams often face challenges in navigating through unfamiliar areas or having issues with SatNav systems which results in misdirecting and confusing them as they urgently required to carry patients into hospitals, leading to potential delays. GeoFindr was created to address this problem by offering an immersive learning experience where drivers can test and improve their location recognition skills in a low-pressure, game-like environment, so that the muscle memory is improved and next time they answer an emergency call, they can respond and arrive to the area faster with minimal stress for both the drivers to understand where they are as well as the patients who are awaiting for their arrival. Moreover, This version of a Geoguesser proved to be an effective way to train ambulance drivers on recognising the areas and everything around them.


🛠️ Technologies Used

Frontend: JavaScript, HTML, CSS
Backend: Rust (server-side development)
Additional tools: Various mapping, geolocation & Google APIs


🏥 How It Works

- The game presents users with a randomly selected street-level image from the Portsmouth area.
- The player (ambulance driver) must reach the location on an interactive map. (Same for Geoguesser). The difference here is that the destination image will be on the righ-hand side of the page and through the interactive map, the player must navigate their way towards the destination as presented on an image.
- To help players understand if they're getting closer or further away from the destination, a meters count is also shown on the left-side of the page, showing how far away is the player from the shown destination.
- Points are awarded based on accuracy (i.e. amount of clicks on interactive map) as well as the time taken to reach a destination. This reinforces geographic awareness.
- Over time, repeated play enhances familiarity with critical locations, aiding real-world navigation.

🔒 Hosting & Security
Previously hosted online, GeoFindr was temporarily taken down due to security considerations. Future iterations will focus on addressing these concerns while improving performance and accessibility.
