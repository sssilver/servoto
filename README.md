# Waldo Photos Engineering Project Solution
This project is a solution to the Waldo engineering assignment described
[here](https://gist.github.com/alwaysunday/db0b32f5ce0538afbb75ccf143adf116).

## Gory details
 - The application is entirely written in Rust, and can be compiled using the
 current **nightly** version of rustc (1.15.x at the time of this writing). Note that
 the nightly limitation is imposed by *Serde*, and not the core codebase
 - The codebase is roughly ~350 lines
 - The service uses a combination of asynchronous I/O using epoll + a pool of
 1--`env::var("WALDO_THREADS")` threads to maximize performance (see _main.rs_)
 - Persistence is achieved through MongoDB (see _storage.rs_)
 - Each thread maintains its own connection to MongoDB
 - `GET /<photo_key>` returns a JSON-ish response with the photo information,
 if the requested key exists
  - Otherwise, it responds with an error message and 404 status
 - `POST /` fetches the XML file from HTTP using libcurl and dumps it into MongoDB 
 - POST spawns threads equal to the number of CPU cores which start downloading and
 processing the EXIF information on the photos. They read from a shared Michael-Scott
  lock-free queue that's fed by the XML listing the photo resources (see _process.rs_)
 - Errors come back with HTTP 500
 
## Installation
 - [Install Rust **Nightly**](https://www.rust-lang.org/en-US/downloads.html)
 - Install & launch MongoDB on `127.0.0.1:27017`, or adjust the config in _main.rs:47_
  - The application will create everything in a collection called `waldo_assignment_areg`
  (See _storage.rs:17_)
 - Install _gexiv2_, the Gnome photo metadata library. On macOS, this is just
 `brew install gexiv2`.
 - In terminal, navigate to the root directory of the project
 - `$ cargo run`
 - Witness that a *GET* request to *http://127.0.0.1:3000/* says "Photo not found",
 which means the server is up and running
 - Do a *POST* to *http://127.0.0.1:3000/* to pull the XML and populate Mongo
 - Send a *GET* to *http://127.0.0.1:3000/<photo_key_id>* and observe the response
  
 *Note:* In order to have the app spawn more than 1 processing thread, set an
 environment variable called `WALDO_THREADS` to the desired number of threads.
 Example: `$ export WALDO_THREADS="4"`.
 
 
## Technical reasoning
### Why Rust?
 - Why not?
 - It's super performant, has a state-of-the-art type system, is a lot of fun to write in
 - Could have implemented this in Python/{gevent/Twisted} in a few hours, but wanted to
 demonstrate I could ship working code in a language I only know the theory of
 - If not good enough, can still ship in Python!
 
### Why MongoDB?
 - Initially the plan was to roll with Redis. Although it doesn't really have "traditional"
 persistence, its snapshotted persistence would be good enough for this project, and much
 faster than MongoDB
 - Since we already have the source XML on S3, Redis with its pub/sub for monitoring changes
 would have been perfect
 - Unfortunately ran into issues with the Redis Rust library, which is super raw at this point
 - Quickly refactored `storage::Storage` to use MongoDB instead
 
### What would you do better?
Given enough time, everything :) In order of priority:
 1. Make `Photo::last_modified` more strongly typed, similar to `Photo::StorageClass`.
 2. Move the feed part out of the current `POST` endpoint. Perhaps make it a separate
 service. If not, spawn it in its own separate thread/process
 3. Switch from DOM-based XML parsing to event-based stream parsing. Uses less memory and
 in this particular instance would also take fewer CPU cycles, albeit is more work
 4. ~~Use [Serde](https://github.com/serde-rs/serde) for serializing/deserializing both XML
 and BSON and JSON~~
 5. ~~Speaking of which, respond with some kind of a proper JSON protocol~~
 6. Dockerize the service to streamline deployment
 7. Externalize configuration
 8. Although error handling is ~~nearly~~ perfect, add proper logging
 9. Unit test where appropriate, although this particular codebase is already pretty fool proof


<sub><sup>
THE FINE PRINT: The sheer amount of fun and learning that happened during this implementation
was over the roof. Two red bull cans were harmed while coding the project. It's entirely
possible that the person writing the problem description had something completely different
in mind, which wouldn't obsolete neither the fun nor the learning mentioned 2 sentences ago.
Rust is distributed under the terms of both the MIT license and the Apache License
(Version 2.0). Every package the project depends on is distributed under similarly
permissive licenses. For questions or concerns, contact sssilver@gmail.com.
</sup></sub>
