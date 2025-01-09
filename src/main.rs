use std::{collections::HashMap, result};
use regex::Regex;
use config::Config;
use telnet::Telnet;

fn main() {
    // basic program configurations
    let server: String = String::from("127.0.0.1");
    let port = 4444;
    let config_path = "config.ini";
    let config_profiles = "Bookmarks_1";

    // create item to gather config data from mobaxterm
    let config = Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .unwrap();

    // returns a hashmap of key/value pairs in the config
    let config: HashMap<String, String> = config.get(config_profiles).unwrap();

    println!("Found profiles:");
    // let results: Vec<(String, String, i32)> = config
    //     .iter()
    //     .filter_map(|(key, value)| get_profile(key, value)) // Only keep matches
    //     .collect();
    // for (device, ip, port) in results {
    //     println!("{} {} {}", device, ip, port);
    // }

    

    // let mut connection = Telnet::connect((server, port), 256)
    //     .expect("Couldn't connect to the server...");

    // loop {
    //     // connection.write(data).expect("Write Error");

    //     let event = connection.read().expect("Read Error");
    //     println!("{:?}", event);

    // }

}

fn get_profile(key: &str, value: &str) -> Option<(String, String, i32)> {
    let regex_ip = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
    let regex_port_and_ip= Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}%\d+").unwrap();
    let ip = regex_ip.find(value).unwrap().as_str().trim().to_string();
    let port = regex_port_and_ip.find(value).unwrap().as_str().trim().split('%').nth(1);
    return Some((key.to_string(), ip, port.unwrap().parse().unwrap()));
}
