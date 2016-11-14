# Waldo Photos Engineering Project Solution
This project is a solution to the Waldo engineering assignment described [here](https://gist.github.com/alwaysunday/db0b32f5ce0538afbb75ccf143adf116).

## Gory details
 - The application is entirely written in Rust, and can be compiled using the current stable version of rustc (1.13.0 at the time of this writing).
 - The codebase is roughly ~350 lines
 - The service uses asynchronous I/O + a pool of 1---`env::var("THREADS")` threads to maximize performance (see _main.rs_)
 - Persistence is achieved through MongoDB (see _storage.rs_)
 - Each thread maintains its own connection to MongoDB
 - `GET /<photo_key>` returns a JSON-ish response with the photo information, if the requested key exists
  - Otherwise, it responds with an error message and 404 status
 - `POST /` fetches the XML file from HTTP using libcurl and dumps it into MongoDB 
 - Errors come back with HTTP 500
 
## Technical reasoning
### Why Rust?
 - Why not?
 - It's super performant, has a state-of-the-art type system, is a lot of fun to write in
 - Could have implemented this in Python/{gevent/Twisted} in a few hours, but wanted to demonstrate I could ship working code in a language I only know the theory of
 - If not good enough, can still ship in Python!
 
### Why MongoDB?
 - Initially the plan was to roll with Redis. Although it doesn't really have "traditional" persistence, its snapshotted persistence would be good enough for this project, and much faster than MongoDB
 - Since we already have the source XML on S3, Redis with its pub/sub for monitoring changes would have been perfect
 - Unfortunately ran into issues with the Redis Rust library, which is super raw at this point
 - Quickly refactored `storage::Storage` to use MongoDB instead
 
### What would you do better?
Given enough time, everything :) In order of priority:
 1. Make `Photo::last_modified` more strongly typed 
 2. Move the feed part out of the current `POST` endpoint. Perhaps make it a separate service. If not, spawn it in its own separate thread/process
 3. Switch from DOM-based XML parsing to event-based stream parsing. Uses less memory and in this particular instance would also take fewer CPU cycles, albeit is more work
 4. Use [Serde](https://github.com/serde-rs/serde) for serializing/deserializing both XML and BSON and JSON
 5. Speaking of which, respond with some kind of a proper JSON protocol
 6. Dockerize the service to streamline deployment
 7. Externalize configuration
 8. Although error handling is ~~nearly~~ perfect, add proper logging
 9. Unit test where appropriate, although this particular codebase is already pretty fool proof

## Disclaimer
Two red bull cans were harmed while shipping this project.
