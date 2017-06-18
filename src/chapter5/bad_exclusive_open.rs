extern crate nix;
extern crate clap;

use nix::{Error, Errno};
use nix::fcntl::*;
use nix::sys::stat::*;
use nix::unistd::*;
use clap::{App, Arg};
use std::process::exit;

fn main() {
    let matches = App::new("bad_exclusive_open")
        .version("1.0")
        .arg(Arg::with_name("file").required(true).index(1))
        .arg(Arg::with_name("sleep").long("sleep"))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let should_sleep = matches.is_present("sleep");

    let fd = open(file, O_WRONLY, Mode::empty());
    match fd {
        Ok(fd) => {
            println!(r#"[PID {}] File "{}" already exists"#, getpid(), file);
            close(fd).expect("Unable to close file");
        }
        Err(_) => {
            match fd {
                Err(Error::Sys(Errno::ENOENT)) => {
                    println!(r#"[PID {}] File "{}" doesn't exist yet"#, getpid(), file);
                    if should_sleep {
                        sleep(5);
                        println!("[PID {}] Done sleeping", getpid());
                    }
                    let fd = open(file, O_WRONLY | O_CREAT, S_IRUSR | S_IWUSR)
                        .expect("Could not create file/open for writing");
                    println!(r#"[PID {}] Created file "{}" exclusively"#, getpid(), file);
                    close(fd).expect("Unable to close file");
                }
                _ => {
                    println!("Failed for unexpected reason");
                    exit(1);
                }
            }
        }
    }
}
