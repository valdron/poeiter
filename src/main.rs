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

    let cl = reqwest::Client::new().expect("client could not be created");
    let res = cl.get("http://api.poe.ninja/api/Data/GetStats").send().expect("get request not successful");
    let val: serde_json::Value = serde_json::from_reader(res).expect("parse error");

    let curr_change_id: String = match *val.get("nextChangeId").unwrap() {
        serde_json::Value::String(ref s) => s.clone(),
        _ => panic!("non string value") 
    };

    let fetcher = PoeFetcher::new("http://www.pathofexile.com/api/public-stash-tabs".parse().unwrap(),Some(curr_change_id));
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
