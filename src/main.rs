extern crate timed_iterator;
extern crate poefetcher;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod poe_json;

use poe_json::ApiSite;

use std::time::{Instant, Duration};
use timed_iterator::TimeIter;
use poefetcher::PoeFetcher;

fn main() {
    let fetcher = PoeFetcher::new("http://api.pathofexile.com/public-stash-tabs".parse().unwrap());
    let res: Result<Vec<ApiSite>, _ > = fetcher.timed(Duration::from_millis(1000))
                            .enumerate()
                            .inspect(|&(ref i, ref site)| println!("Site {}: {:?}", i, site.change_id))
                            .map(|(_, site)| serde_json::from_reader(site.body.as_slice()))
                            .take(100)
                            .collect(); 
    match res {
        Err(e) => println!("{:?}",e),
        Ok(_) => println!("ran correctly"),
    }
    
}
