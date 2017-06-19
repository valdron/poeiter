extern crate timed_iterator;
extern crate regex;
extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod poe_json;
mod poe_rust;
mod poe_fetcher;

use poe_json::ApiSite;

use std::time::Duration;
use timed_iterator::TimeIter;
use poe_fetcher::PoeFetcher;

fn main() {
    let fetcher = PoeFetcher::new("http://www.pathofexile.com/api/public-stash-tabs".parse().unwrap(),Some("70269659-73958331-69202239-80449068-74825503".into()));
    let res: Vec<_> = fetcher
        .timed(Duration::from_millis(1000))
        .enumerate()
        .take(5000)
        .inspect(|&(ref i, ref site)| println!("Site {}: {:?}", i, site.change_id))
        .map(|(_, site)| serde_json::from_slice::<ApiSite>(&site.body))
        .filter(|res| res.is_err())
        .collect();
    
    println!("{:#?}",res);
}
