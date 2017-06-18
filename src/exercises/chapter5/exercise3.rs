extern crate nix;
extern crate clap;

use nix::fcntl::*;
use nix::sys::stat::*;
use nix::unistd::*;
use clap::{App, Arg};

fn main() {
    let matches = App::new("exercise3")
        .version("1.0")
        .arg(Arg::with_name("file").required(true).index(1))
        .arg(Arg::with_name("num-bytes").required(true).index(2))
        .arg(Arg::with_name("seek").long("seek"))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let num_bytes = matches
        .value_of("num-bytes")
        .unwrap()
        .parse::<u64>()
        .expect("Error parsing num-bytes");
    let should_seek = matches.is_present("seek");

    let common_flags = O_WRONLY | O_CREAT;
    let flags = if should_seek {
        common_flags
    } else {
        common_flags | O_APPEND
    };

    let fd = open(file, flags, S_IRUSR | S_IWUSR).expect("Could not open file");

    for _ in 0..num_bytes {
        if should_seek {
            lseek(fd, 0, Whence::SeekEnd).expect("Could not seek");
        }
        write(fd, "a".as_bytes()).expect("Could not write to file");
    }

    close(fd).expect("Could not close file");
}
