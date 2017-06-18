extern crate nix;
extern crate clap;

use nix::fcntl::*;
use nix::sys::stat::*;
use nix::unistd::*;
use clap::{App, Arg};
use nix::libc::off_t;

fn main() {
    let matches = App::new("t_truncate")
        .version("1.0")
        .arg(Arg::with_name("file").required(true).index(1))
        .arg(Arg::with_name("length").required(true).index(2))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let length = matches
        .value_of("length")
        .unwrap()
        .parse::<off_t>()
        .expect("Error parsing length");

    let fd = open(file, O_RDWR, Mode::empty()).expect("Could not open file");

    ftruncate(fd, length).expect("Could not truncate file");

    close(fd).expect("Could not close file");
}
