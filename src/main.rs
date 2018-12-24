mod protocol_generated;

use crate::protocol_generated::ckb::protocol::{Bytes, BytesBuilder, ScriptBuilder};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::io::{self, Read, Write};

fn main() {
    let mut data = vec![];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut data).unwrap();

    let mut fbb = FlatBufferBuilder::new();

    let seq = fbb.create_vector(&data);
    let mut bytes_builder = BytesBuilder::new(&mut fbb);
    bytes_builder.add_seq(seq);
    let binary = bytes_builder.finish();

    let args: Vec<WIPOffset<Bytes>> = Vec::new();
    let args = fbb.create_vector(&args);
    let signed_args: Vec<WIPOffset<Bytes>> = Vec::new();
    let signed_args = fbb.create_vector(&signed_args);

    let mut script_builder = ScriptBuilder::new(&mut fbb);
    script_builder.add_version(0);
    script_builder.add_args(args);
    script_builder.add_binary(binary);
    script_builder.add_signed_args(signed_args);
    let script_offset = script_builder.finish();
    fbb.finish(script_offset, None);

    io::stdout().write(fbb.finished_data()).unwrap();
}
