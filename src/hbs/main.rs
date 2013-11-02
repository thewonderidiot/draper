#[link(name = "hbs",
       vers = "0",
       url  = "http://github.com/thewonderidiot/hbs")];
#[desc = "A rustic AGC emulator"];
#[license = "MIT"];
#[crate_type = "bin"];

extern mod extra;

use extra::getopts::{getopts, optflag};
use std::path::Path;
use std::rt::io;
use std::rt::io::Reader;
use std::rt::io::file::FileInfo;
use std::os;

fn print_usage() {
    println("Usage: hbs [OPTIONS] INPUT\n");
    println("Options:");
    println("    -h --help    Display this message");
    println("");
}

pub struct Rom([u8, ..0x9000]);

fn main() {
    let args = os::args();
    if args.len() == 1 {
        print_usage();
        return;
    }

    let opts = [
        optflag("h"),
        optflag("help")
    ];
    let matches = getopts(args.tail(), opts).unwrap();

    if matches.opt_present("h") || matches.opt_present("help") {
        print_usage();
        return;
    }

    let file = match matches.free.len() {
        0u => fail!("no ROM file supplied"),
        1u => Path::new(matches.free[0]),
        _  => fail!("multiple input filenames")
    };

    if !file.exists() {
        fail!("file doesn't exist");
    }

    let mut reader = file.open_reader(io::Open).unwrap();

    let mut buf = [0u8, ..0x9000];

    reader.read(buf);

    for i in buf.iter() {
        print!("{:02x}", *i);
    }
}
