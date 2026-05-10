#!/usr/bin/liquidsoap

# Radio station configuration
settings.server.socket.set(true)
settings.server.socket.path.set("/tmp/liquidsoap.sock")

# Music source - playlist from /music directory
music = playlist(
  reload=3600,
  reload_mode="watch",
  "/music"
)

# Fallback to silence if no music
source = fallback(track_sensitive=false, [music, blank()])

# Output to Icecast
output.icecast(
  %mp3(bitrate=128),
  host="icecast",
  port=8000,
  password="hackme",
  mount="/radio",
  name="home.sh Radio",
  description="The home.sh community radio station",
  url="http://home.sh",
  source
)
