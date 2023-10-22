#![no_std]
use gstd::prelude::*;
use gmeta::{In, InOut, Metadata};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<String, String>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}

