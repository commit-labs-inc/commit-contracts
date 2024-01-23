#![no_std]
use gstd::prelude::*;
use gmeta::{In, InOut, Metadata};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitMTK>;
    type Handle = InOut<MTKAction, MTKEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum InitMTK {

}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKAction {

}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKEvent {

}