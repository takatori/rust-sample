extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

pub struct LineCode;

impl Codec for LineCode {
    type In = String;
    type Out = String;
}

fn main() {
    println!("Hello, world!");
}
