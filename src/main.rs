use std::collections::HashMap;
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
    let results: Vec<(String, String)> = config
        .iter()
        .filter_map(|(key, value)| get_profile(key, value)) // Only keep matches
        .collect();

    for (device, ip) in results {
        println!("{} {}", device, ip);
    }

    

    let mut connection = Telnet::connect((server, port), 256)
        .expect("Couldn't connect to the server...");

    loop {
        // connection.write(data).expect("Write Error");

        let event = connection.read().expect("Read Error");
        println!("{:?}", event);

    }

}

fn get_profile(key: &str, value: &str) -> Option<(String, String)> {
    let regex = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
    if let Some(mat) = regex.find(value) {
        let ip = mat.as_str().trim().to_string();
        return Some((key.to_string(), ip)); // Return the device and IP as Some
    }
    None // Return None if no match is found
}
