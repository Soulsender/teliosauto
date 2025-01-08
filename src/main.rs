use telnet::Telnet;


fn main() {

    let data = "hello world\nasdasd\n";
    let server: String = String::from("127.0.0.1");
    let buffer= data.as_bytes();

    use telnet::Telnet;

    let mut connection = Telnet::connect((server, 4444), 256)
            .expect("Couldn't connect to the server...");

    loop {
        connection.write(buffer).expect("Write Error");

        let event = connection.read().expect("Read Error");
        println!("{:?}", event);
    }

}
