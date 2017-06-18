extern crate nix;
extern crate clap;

use nix::fcntl::*;
use nix::sys::stat::*;
use nix::unistd::*;
use clap::{App, Arg};

const BUF_SIZE: usize = 1024;

fn main() {
    let mut buf = [0u8; BUF_SIZE];

    let matches = App::new("copy")
        .version("1.0")
        .arg(Arg::with_name("old-file").required(true).index(1))
        .arg(Arg::with_name("new-file").required(true).index(2))
        .get_matches();

    let input_filename = matches.value_of("old-file").unwrap();
    let output_filename = matches.value_of("new-file").unwrap();

    let input_fd = open(input_filename, O_RDONLY, Mode::empty())
        .expect("Could not open input file");

    let open_flags = O_CREAT | O_WRONLY | O_TRUNC;
    let file_perms = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
    let output_fd = open(output_filename, open_flags, file_perms)
        .expect("Could not open output file");

    while let Ok(n) = read(input_fd, &mut buf[..]) {
        if n == 0 {
            break;
        }
        let m = write(output_fd, &buf[..n]).expect("Could not write");
        if m != n {
            panic!("couldn't write whole buffer");
        }
    }

    close(input_fd).expect("Could not close input file");
    close(output_fd).expect("Could not close output file");
}
