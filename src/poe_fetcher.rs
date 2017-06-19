

use reqwest::{Client, Url};
use reqwest::header::{Headers, ContentEncoding, Encoding};
use reqwest::Method;
use regex::bytes::Regex;
use std::io::Read;

#[derive(Debug)]
pub struct PoeSite {
    pub change_id: String,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct PoeFetcher {
    url: Url,
    client: Client,
    next_id: String,
}


impl PoeFetcher {
    pub fn new(start_url: Url, start_id: Option<String>) -> Self {
        Self {
            url: start_url,
            client: Client::new().unwrap(),
            next_id: start_id.unwrap_or("0-0-0-0-0".into()),
        }
    }
}

impl Iterator for PoeFetcher {
    type Item = PoeSite;
    fn next(&mut self) -> Option<Self::Item> {

        self.url
            .query_pairs_mut()
            .clear()
            .append_pair("id", &self.next_id);

        let mut headers = Headers::new();
        headers.set(ContentEncoding(vec![Encoding::Gzip]));

        let response = self.client
            .request(Method::Get, self.url.clone())
            .headers(headers)
            .send();



        match response {
            Ok(mut res) => {

                let mut result = Vec::with_capacity(5_000_000);
                let old_id = self.next_id.clone();

                let size = res.read_to_end(&mut result);

                match size {
                    Ok(x) if x == 0 => return None,
                    Err(e) => return None,
                    _ => {}
                }
                self.next_id = match extract_next_id(&result) {
                    Ok(x) => x,

                    Err(e) => return None,
                };

                if old_id == self.next_id {
                    println!("Got Same again!");
                }

                Some(PoeSite {
                         change_id: old_id,
                         body: result,
                     })
            }

            _ => None,
        }
    }
}

fn extract_next_id(s: &[u8]) -> Result<String, String> {
    let re = Regex::new("\\{\"next_change_id\":\"((?:\\d|-)+)\",").unwrap();
    let mat = match re.captures(s).and_then(|cap| cap.get(1)) {
        Some(x) => x,
        None => return Err("no changeid found in body".into()),
    };
    String::from_utf8(mat.as_bytes().to_vec()).map_err(|_| "no utf8".into())
}
