use std::{collections::HashMap, fs::File, io::BufRead, net::{IpAddr, SocketAddr}, result};
use regex::{Match, Regex};
use config::Config;
use telnet::Telnet;
use std::io;

fn main() {
    // basic program configurations
    let config_path = "config.ini";
    let bookmarks = "Bookmarks_1";

    // create item to gather config data from mobaxterm
    let config = Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .expect("Unable to load configuration file");

    // returns a hashmap of key/value pairs in the config
    let config: HashMap<String, String> = config.get(bookmarks).expect("Unable to find bookmarked devices in config file");
    

    println!("Found profiles:");
    for (key, value) in config {
        
        let profile = match get_profile(key, value) {
            Ok(profile) => profile,
            Err(_) => continue,
        };
        let (name, ip, port) = profile;
        if (port == 0) && (ip.is_empty()) {
            continue;
        } else {
            println!("Device {} {}:{} found", name, ip, port);
            let path = format!("config\\{}.txt", name);
            let file = match File::open(path.clone().trim()) {
                Ok(path) => {println!("Opening {}", name); path},
                Err(_) => {eprintln!("Failed to open file {path}: skipping..."); continue;},
            };
            let mut connection = Telnet::connect(SocketAddr::new(ip.parse().unwrap(), port.try_into().unwrap()), 1024)
                .expect("Couldn't connect to the server...");
            let reader = io::BufReader::new(file);
            for x in reader.lines() {
                let line = x.unwrap();
                connection.write(line.as_bytes()).expect("Write Error");
            }
        }
    }
}

fn get_profile(key: String, value: String) -> Result<(String, String, i32), io::Error> {
    let regex_ip = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
    let regex_port_and_ip= Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}%\d+").unwrap();
    let ip = regex_ip.find(&value).unwrap_or_else(|| {
        eprintln!("Invalid profile IP {key}, skipping...");
        Regex::new(r"").unwrap().find("").unwrap()
    }).as_str().trim().to_string();
    let port = regex_port_and_ip.find(&value).unwrap_or_else(|| {
        eprintln!("Invalid profile port {key}, skipping...");
        Regex::new(r"").unwrap().find("").unwrap()
    }).as_str().trim().split('%').nth(1).unwrap_or_default().parse().unwrap_or_default();
    Ok((key.to_string().replace(" ", "_"), ip, port))
}
