use std::{env, process};
use std::process::Command;

fn main() {
    let args = env::args();
    let open = handle(args).unwrap_or_else(|message| {
        eprintln!("Unable to launch: {}", message);
        process::exit(1);
    });
    open.run()
}

const APPLICATION_FOLDER: &str = "/Applications/";
const DEFAULT_APP_EXTENSION: &str = ".app";
const OS_NAME: &str = env::consts::OS;

enum OperatingSystem {
    MacOs,
    Windows,
    Linux,
}

struct Opener {
    os: OperatingSystem,
    app: String,
}

impl Opener {
    fn run(&self) {
        let location = String::from(APPLICATION_FOLDER.to_owned() + &self.app + DEFAULT_APP_EXTENSION);

        let command = match &self.os {
            OperatingSystem::MacOs => String::from("open"),
            OperatingSystem::Windows => String::from("start"),
            OperatingSystem::Linux => String::from("xdg-open"),
        };

        println!("{} {}", command, location);

        Command::new(command)
            .arg(location)
            .output()
            .expect("Failed to open application");
    }
}

fn handle(mut args: impl Iterator<Item=String>) -> Result<Opener, &'static str> {
    args.next();

    let app = match args.next() {
        Some(arg) => String::from(arg),
        None => return Err("invalid app or file argument"),
    };

    let os = get_os();
    Ok(Opener { os, app })
}

fn get_os() -> OperatingSystem {
    if OS_NAME.to_lowercase().contains("macos") {
        OperatingSystem::MacOs
    } else if OS_NAME.to_lowercase().contains("windows") {
        OperatingSystem::Windows
    } else {
        OperatingSystem::Linux
    }
}
