extern crate nix;
extern crate clap;

use nix::fcntl::*;
use nix::sys::stat::*;
use nix::unistd::*;
use clap::{App, Arg};

fn main() {
    let matches = App::new("seek_io")
        .version("1.0")
        .arg(Arg::with_name("file").required(true).index(1))
        .arg(Arg::with_name("action").required(true).multiple(true))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let actions = matches.values_of("action").unwrap();

    let fd = open(
        file,
        O_RDWR | O_CREAT,
        S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH,
    ).expect("Could not open or create file");

    for action in actions {
        match action.chars().next() {
            Some(mode) if mode == 'r' || mode == 'R' => {
                let len = action[1..].parse::<usize>().expect("Error parsing offset");
                let mut buf = vec![0; len];

                let num_read = read(fd, &mut buf[..]).expect("Could not read from file");

                if num_read == 0 {
                    println!("{}: end-of-file", action);
                } else {
                    print!("{}: ", action);
                    if mode == 'r' {
                        print!("{}", String::from_utf8_lossy(&buf[..num_read]));
                    } else {
                        for byte in &buf[..num_read] {
                            print!("{:02x} ", byte);
                        }
                    }
                    println!();
                }
            }
            Some('w') => {
                let num_written = write(fd, action[1..].as_bytes())
                    .expect("Could not write to file");
                println!("{}: wrote {} bytes", action, num_written);
            }
            Some('s') => {
                let offset = action[1..].parse::<i64>().expect("Error parsing offset");
                lseek(fd, offset, Whence::SeekSet).expect("Could not change file offset");
                println!("{}: seek succeeded", action);
            }
            _ => println!("Argument must start with [rRws]: {}", action),
        }
    }

    close(fd).expect("Could not close file")
}
