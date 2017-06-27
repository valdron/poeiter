#![feature(try_from)]

extern crate timed_iterator;
extern crate regex;
extern crate reqwest;
#[macro_use]
extern crate error_chain;


extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod errors {
    error_chain!{
        foreign_links {
                ParseInt(::std::num::ParseIntError);
        }
        errors {
            InvalidRequirementName(s: String) {
                description("invalid requirment name")
                display("invalid requirement name: {}", s)
            }
            RequirementArrayEmpty {
                description("Requirement Array Empty")
                display("Requirement Array Empty")
            }
            InvalidRequirementValue(s: String) {
                description("Invalid Requirement Value")
                display("Invalid Requirement Value: {}", s)
            }

        }
    }
}



mod poe_json;
mod poe_rust;
mod poe_fetcher;
mod poe_item_types;

use poe_json::ApiSite;

use std::time::Duration;
use timed_iterator::TimeIter;
use poe_fetcher::PoeFetcher;
use poe_rust::FrameType;
use poe_item_types::CurrencyType;
use std::convert::TryInto;
use errors::*;


fn main() {

    let cl = reqwest::Client::new().expect("client could not be created");
    let res = cl.get("http://api.poe.ninja/api/Data/GetStats")
        .send()
        .expect("get request not successful");
    let val: serde_json::Value = serde_json::from_reader(res).expect("parse error");

    let curr_change_id: String = match *val.get("nextChangeId").unwrap() {
        serde_json::Value::String(ref s) => s.clone(),
        _ => panic!("non string value"),
    };

    let fetcher = PoeFetcher::new(
        "http://www.pathofexile.com/api/public-stash-tabs"
            .parse()
            .unwrap(),
        Some(curr_change_id),
    );
    let res: Vec<_> = fetcher
        .timed(Duration::from_millis(1000))
        .enumerate()
        .inspect(|&(ref i, ref site)| {
            println!("Site {}: {:?}", i, site.change_id)
        })
        .map(|(_, site)| serde_json::from_slice::<ApiSite>(&site.body))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .flat_map(|site| site.stashes.into_iter())
        .flat_map(|stash| stash.items.into_iter())
        .filter(|item| {
            let ft: Result<FrameType> = item.frame_type.try_into(); 
            if let Ok(f) = ft {
                f == FrameType::Currency
            } else {
                false
            }
        })
        .take(1000)
        .inspect(|i| println!("typeline: {}", i.type_line))
        .map(|itm| -> CurrencyType { itm.type_line.into() })
        .collect();

    println!("{:#?}", res);
}
