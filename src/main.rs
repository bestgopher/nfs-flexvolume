mod mount;
mod umount;
mod result;
mod utils;

use result::{Response, Status::*};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let r = match args.get(1) {
        Some(i) if i == "mount" => mount::mount(&args),
        Some(i) if i == "umount" => umount::umount(&args),
        Some(i) if i == "init" => {
            println!("{}", Response::new().with_status(Success).with_attach(false));
            std::process::exit(0);
        }
        Some(_) => {
            eprintln!("{}", Response::new().with_status(NotSupported));
            std::process::exit(1);
        }
        None => {
            eprintln!("{}", Response::new().with_status(Failure).with_message("can't find subcommand"));
            std::process::exit(1);
        }
    };

    match r {
        Ok(_) => println!("{}", Response::new().with_status(Success)),
        Err(e) => {
            eprintln!("{}", Response::new().with_status(Failure).with_message(e));
            std::process::exit(1);
        }
    }
}
