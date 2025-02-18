use std::{collections::HashMap, fs::File, io::BufRead, net::SocketAddr, path::Path};
use regex::Regex;
use config::Config;
use telnet::Telnet;
use std::io;
use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true, help = "Path to your MobaXTerm .ini config")]
    config: Option<String>,

    #[arg(short, long, required = true, help = "Path to your device config files\n(ex. ./path/to/devices/)")]
    devices: Option<String>,

    #[arg(short, long, default_value = "Bookmarks", required = false, help = "(Optional) header tag in MobaXTerm .ini config\nBy default searches for \"[Bookmarks]\"")]
    tag: Option<String>,

    #[arg(short, long, default_value = "conf", required = false, help = "(Optional) File extension for your device config\nBy default searches for \".conf\"")]
    extension: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // basic program configurations
    let config_path = args.config.unwrap();
    let x = args.devices.unwrap();
    let device_path = Path::new(x.as_str());
    let bookmarks = args.tag.unwrap();
    let extension = args.extension.unwrap();

    // create item to gather config data from mobaxterm
    let config = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()
        .expect("Unable to load configuration file");

    // returns a hashmap of key/value pairs in the config
    let config: HashMap<String, String> = config.get(&bookmarks).expect("Unable to find bookmarked devices in config file");
    
    for (key, value) in config {
        // for each profile found in config
        // gather name, ip, and port
        let profile = match get_profile(key, value) {
            Ok(profile) => profile,
            Err(_) => continue,
        };
        let (name, ip, port) = profile;

        // if the port is set to default value (0); disregard this iteration
        if port == 0 {
            continue;
        } else {

            // gather path data to device config files
            let path = device_path.join(format!("{}.{}", name, extension));
            let pretty_path = format!("{name} @ {ip}:{port}");

            // open device config file
            let file = match File::open(path) {
                Ok(path) => {println!("Opening {pretty_path} with path {device_path:?}"); path},
                Err(_) => continue,
            };

            // create telnet socket
            let mut connection = match Telnet::connect(SocketAddr::new(ip.parse().unwrap(), port.try_into().unwrap()), 1024) {
                Ok(x) => {println!("Connected to {pretty_path}"); x},
                Err(error) => match error.kind() {
                    std::io::ErrorKind::ConnectionRefused => {eprintln!("The connection to the server was refused!"); return},
                    _ => panic!("Unexpected error {}", error),
                },
            };

            // read lines from device config files and write that data through telnet
            let reader = io::BufReader::new(file);
            for x in reader.lines() {
                let line = x.unwrap();
                connection.write(format!("{line} \n").as_bytes()).expect("Write Error");
            }

            println!("Done")
        }
    }
}

fn get_profile(key: String, value: String) -> Result<(String, String, i32), io::Error> {
    // set the regex for an ip address ex. "127.0.0.1"
    let regex_ip = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
    // set the regex for ip address/port pair ex. "127.0.0.1%4444"
    // this must be done so you do not get a port value from somewhere else in the line; the port always comes after the ip in the config
    let regex_port_and_ip= Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}%\d+").unwrap();
    
    // check if strings match regex
    let ip = regex_ip.find(&value).unwrap_or_else(|| {
        Regex::new(r"").unwrap().find("").unwrap()
    }).as_str().trim().to_string();
    let port = regex_port_and_ip.find(&value).unwrap_or_else(|| {
        Regex::new(r"").unwrap().find("").unwrap()
    }).as_str().trim().split('%').nth(1).unwrap_or_default().parse().unwrap_or_default();

    // return tuple (name: String, ip: String, port: i32)
    Ok((key.to_string().replace(" ", "_"), ip, port))
}
