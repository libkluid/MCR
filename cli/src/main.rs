extern crate config;
extern crate serde;
extern crate rustyline;
extern crate rcon;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    address: String,
    port: u16,
    password: String,
}

fn main() {
    let config: Config = config::Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .expect("Failed to read config from 'config.toml'")
        .try_deserialize()
        .expect("Failed to deserialize 'config.toml'");

    let addr = (config.address, config.port);
    let mut client = rcon::Client::connect(addr, config.password)
        .expect("Failed to connect to server");
    
    let mut editor = rustyline::Editor::<()>::new().expect("Failed to create editor");

    loop {
        match editor.readline(">> ") {
            Ok(command) => {
                editor.add_history_entry(command.as_str());
                execute_command(&mut client, command);
            }
            Err(rustyline::error::ReadlineError::Interrupted) => eprintln!("Interrupted: <CTRL-C>"),
            Err(rustyline::error::ReadlineError::Eof) => {
                eprintln!("EOF");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn execute_command(client: &mut rcon::Client, command: String) {
    match client.execute(command) {
        Ok(msg) => println!("{}", msg),
        Err(err) => eprintln!("ERROR: {}", err),
    }
}
