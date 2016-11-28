use crossbeam::sync::MsQueue;
use download::download;
use error::WaldoError;
use photo::{Photo, PhotoResource};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle, Thread};
use storage::Storage;


pub struct Processor {
    num_threads: usize,
    threads: Vec<JoinHandle<Thread>>,
    queue: Arc<MsQueue<String>>,  // A Michael-Scott queue of photo names to download & process

    database: Arc<Storage>
}


impl Processor {
    pub fn new(storage: Arc<Storage>, num_threads: usize) -> Processor {
        Processor {
            num_threads: num_threads,
            threads: Vec::new(),
            queue: Arc::new(MsQueue::new()),

            database: storage
        }
    }

    pub fn process(&mut self, uri: String) -> Result<(), WaldoError> {
        // Fetch the XML from S3
        download(&uri, |response| -> Result<(), WaldoError> {
            // Parse all the photo resources
            let photo_resources = PhotoResource::new_many(&response)?;

            println!("{:?}", photo_resources);
            println!("Total: {} photos", photo_resources.len());

            // Put the work in the queue
            for photo_resource in photo_resources {
                self.queue.push(photo_resource.key);
            }

            // Spawn the threads
            for thread_id in 0..self.num_threads {
                // Clone a reference to the queue
                let thread_queue = self.queue.clone();
                let database = self.database.clone();
                let uri = uri.clone();

                self.threads.push(spawn(move || {
                    println!("Spawned thread {}", thread_id);

                    // Try popping from the queue; block if empty
                    loop {
                        let photo_key = thread_queue.pop();
                        let url = uri.clone() + "/" + &photo_key;

                        if let Err(err) = download(&url, |response| -> Result<(), WaldoError> {
                            // Photo data is downloaded; parse the photo
                            let photo = Photo::new(&photo_key, response)?;

                            // ...and shove it into our storage
                            database.store_one(photo)?;

                            Ok(())
                        }) {
                            println!("Error parsing or downloading photo {}: {}", photo_key, err);
                            continue;
                        }

                        println!("Item for thread {}!!", thread_id);
                    }
                }));
            }

            Ok(())
        })
    }
}