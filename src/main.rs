extern crate clap;
extern crate hyper;

use clap::{Arg, App};
use std::collections::HashMap;

fn parse_url(query: &str) -> HashMap<String, String> {
    let url = format!("http://e.com?{}", query);
    let parsed_url = hyper::Url::parse(&url).unwrap();
    parsed_url.query_pairs()
            .into_owned()
            .collect()
}

fn main() {
	let arguments = App::new("url-decoder")
	                    .version("0.1.0")
	                    .author("Ritiek Malhotra <ritiekmalhotra123@gmail.com>")
	                    .about("CLI tool to decode URL(s) back to readable form.")

	                    .arg(Arg::with_name("urls")
                        .multiple(true)
                        .required(true)
	                    .help("URL(s) to decode"))

						.get_matches();

	let raw_urls = arguments.values_of("urls");
	for raw_url in raw_urls.unwrap() {
		let url = format!("url={}", raw_url);

		let result = parse_url(&url);
		println!("\n{}", &result["url"]);
	}
}
