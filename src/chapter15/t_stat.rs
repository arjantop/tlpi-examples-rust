extern crate nix;
extern crate clap;
extern crate libc;
extern crate chrono;

use nix::sys::stat::{FileStat, lstat, stat, major, minor};
use clap::{App, Arg};
use libc::*;
use chrono::prelude::*;

fn file_perm_string(perm: mode_t) -> String {
    fn exec_perm_string(
        perm: mode_t,
        mask: u32,
        special: i32,
        special_str: &'static str,
        special_only_str: &'static str,
    ) -> &'static str {
        if perm & mask as u32 != 0 {
            if perm & special as u32 != 0 {
                special_str
            } else {
                "x"
            }
        } else {
            if perm & special as u32 != 0 {
                special_only_str
            } else {
                "-"
            }
        }
    }

    format!(
        "{}{}{}{}{}{}{}{}{}",
        if perm & S_IRUSR != 0 { "r" } else { "-" },
        if perm & S_IWUSR != 0 { "w" } else { "-" },
        exec_perm_string(perm, S_IXUSR, S_ISUID, "s", "S"),
        if perm & S_IRGRP != 0 { "r" } else { "-" },
        if perm & S_IWGRP != 0 { "w" } else { "-" },
        exec_perm_string(perm, S_IXGRP, S_ISGID, "s", "S"),
        if perm & S_IROTH != 0 { "r" } else { "-" },
        if perm & S_IWOTH != 0 { "w" } else { "-" },
        exec_perm_string(perm, S_IXOTH, S_ISVTX, "s", "S")
    )
}

fn str_if(cond: bool, s: &'static str) -> &'static str {
    if cond { s } else { "" }
}

fn display_stat_info(fs: &FileStat) {
    let ft = fs.st_mode & S_IFMT;
    let file_type = if ft == S_IFREG {
        "regular file"
    } else if ft == S_IFDIR {
        "directory"
    } else if ft == S_IFCHR {
        "character device"
    } else if ft == S_IFBLK {
        "block device"
    } else if ft == S_IFLNK {
        "symbolic (soft) link"
    } else if ft == S_IFIFO {
        "FIFO or pipe"
    } else if ft == S_IFIFO {
        "socket"
    } else {
        "unknown file type?"
    };
    println!("File type:                {}", file_type);

    println!(
        "Device containing i-node: major={}   minor={}",
        major(fs.st_dev),
        minor(fs.st_dev)
    );

    println!("I-node number:            {}", fs.st_ino);

    println!(
        "Mode:                     {} ({})",
        fs.st_mode,
        file_perm_string(fs.st_mode)
    );

    if fs.st_mode & (S_ISUID | S_ISGID | S_ISVTX) as u32 != 0 {
        let set_uid = str_if(fs.st_mode & S_ISUID as u32 != 0, "set-UID ");
        let set_gid = str_if(fs.st_mode & S_ISGID as u32 != 0, "set-GID ");
        let sticky = str_if(fs.st_mode & S_ISVTX as u32 != 0, "sticky ");
        println!("    special bits set:     {}{}{}", set_uid, set_gid, sticky)
    }

    println!("Number of (hard) links:   {}", fs.st_nlink);

    println!(
        "Ownership:                UID={}   GID={}",
        fs.st_uid,
        fs.st_gid
    );

    if ft == S_IFCHR || ft == S_IFBLK {
        println!(
            "Device number (st_rdev):  major={}; minor={}",
            major(fs.st_rdev),
            minor(fs.st_rdev)
        );
    }

    println!("File size:                {} bytes", fs.st_size);
    println!("Optimal I/O block size:   {} bytes", fs.st_blksize);
    println!("512B blocks allocated:    {}", fs.st_blocks);

    println!(
        "Last file access:         {}",
        Local.timestamp(fs.st_atime, 0).to_rfc2822()
    );
    println!(
        "Last file modification:   {}",
        Local.timestamp(fs.st_mtime, 0).to_rfc2822()
    );
    println!(
        "Last status change:       {}",
        Local.timestamp(fs.st_ctime, 0).to_rfc2822()
    );
}

fn main() {
    let matches = App::new("t_stat")
        .version("1.0")
        .arg(Arg::with_name("file").required(true).index(1))
        .arg(Arg::with_name("lstat").short("l"))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let stat_link = matches.is_present("lstat");

    let result = if stat_link {
        lstat(file).expect("Could ot lstat file")
    } else {
        stat(file).expect("Could not stat file")
    };

    display_stat_info(&result);
}
