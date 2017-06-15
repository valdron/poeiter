extern crate timed_iterator;
extern crate poefetcher;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod poe_json;

use poe_json::ApiSite;

use std::fs::File;
use std::io::Write;
use std::time::{Instant, Duration};
use timed_iterator::TimeIter;
use poefetcher::PoeFetcher;

fn main() {
    let fetcher = PoeFetcher::new("http://localhost:8000/public-stash-tabs".parse().unwrap());
    let res: Vec<_> = fetcher.timed(Duration::from_millis(1000))
                            .enumerate()
                            .inspect(|&(ref i, ref site)| println!("Site {}: {:?}", i, site.change_id))
                            .map(|(_, site)| serde_json::from_reader::<&[u8],ApiSite>(site.body.as_slice()))
                            .flat_map(| s | s.unwrap().stashes.into_iter())
                            .flat_map(|stash| stash.items.into_iter())
                            .take(10000)
                            .filter(| i | i.ilvl == 84)
                            .collect(); 
    println!("len: {}", res.len());
    
}
