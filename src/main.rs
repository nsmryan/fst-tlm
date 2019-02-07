extern crate fst;
extern crate byteorder;

use std::env;
use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use std::mem;

use fst::{Set, SetBuilder, Streamer, IntoStreamer};

use byteorder::{ByteOrder, BigEndian};


fn main() {
    let args: Vec<String> = env::args().collect();

    let record_length_bytes = args[1].parse::<usize>().unwrap();
    let mut file = File::open(&args[2]).unwrap();

    let mut bytes = vec![0; record_length_bytes];
    let mut with_index = vec![0; record_length_bytes + mem::size_of::<u64>()];
    let entry_length_bytes = record_length_bytes + mem::size_of::<u64>();

    println!("{:?}", args);

    let mut builder = SetBuilder::memory();

    let mut index: u64 = 0;

    while let Ok(()) = file.read_exact(&mut bytes) {
        println!("loop {}", index);
        println!("bytes = {:?}", bytes);

        if bytes.len() == 0 {
            break;
        }

        BigEndian::write_u64(&mut with_index, index);

        with_index.append(&mut bytes);

        builder.insert(with_index.clone());

        index += 1;

        for index in 0..record_length_bytes {
            bytes.push(0);
        }

        for index in 0..entry_length_bytes {
            with_index[index] = 0;
        }
    }

    let bytes = builder.into_inner().unwrap();

    let num_bytes = bytes.len();
    println!("Set contained {} bytes", num_bytes);

    let set = Set::from_bytes(bytes).unwrap();
    let mut stream = set.into_stream();
    let mut keys = vec!();
    while let Some(key) = stream.next() {
        keys.push(key.to_vec());
        //println!("key = {:?}", key);
    }
}
