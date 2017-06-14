extern crate timed_iterator;
extern crate poefetcher;



use std::time::{Instant, Duration};
use timed_iterator::TimeIter;
use poefetcher::PoeFetcher;

fn main() {
    let fetcher = PoeFetcher::new("http://api.pathofexile.com/public-stash-tabs".parse().unwrap());
    for (i, site) in fetcher.timed(Duration::from_millis(1000)).enumerate().take(100) {
        println!("Site {}: {:?}", i, site.change_id);
    }
}
