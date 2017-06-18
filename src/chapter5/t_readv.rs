extern crate nix;
extern crate clap;

use nix::sys::uio::*;
use nix::fcntl::*;
use nix::sys::stat::*;
use nix::unistd::*;
use nix::sys::stat::FileStat;
use clap::{App, Arg};

use std::mem::{zeroed, size_of};
use std::slice::from_raw_parts_mut;

unsafe fn as_mut_slice<'a, T>(v: &'a mut T) -> &'a mut [u8] {
    let v_mut = v as *mut T;
    from_raw_parts_mut(v_mut as *mut u8, size_of::<T>())
}

fn main() {
    let mut my_struct: FileStat = unsafe { zeroed() };
    let mut x = 0i32;
    let mut str = [0u8; 100];

    let tot_required = size_of::<FileStat>() + size_of::<i32>() + str.len() + size_of::<u8>();

    let matches = App::new("t_readv")
        .version("1.0")
        .arg(Arg::with_name("file").required(true).index(1))
        .get_matches();

    let file = matches.value_of("file").unwrap();

    let fd = open(file, O_RDONLY, Mode::empty()).expect("Could not open file");

    let mut iovec = unsafe {
        [
            IoVec::from_mut_slice(as_mut_slice(&mut my_struct)),
            IoVec::from_mut_slice(as_mut_slice(&mut x)),
            IoVec::from_mut_slice(&mut str[..]),
        ]
    };
    let num_read = readv(fd, &mut iovec).expect("Could not read from file");

    if num_read < tot_required {
        println!("Read fewer bytes than requested");
    }

    println!(
        "total bytes requested: {}; bytes read: {}",
        tot_required,
        num_read
    );

    close(fd).expect("Could not close file")
}
