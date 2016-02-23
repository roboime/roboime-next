extern crate protobuf_build;

use std::env;
use std::path::PathBuf;

static PROTO_FILES: &'static [&'static str] = &[
    "grSim_Commands.proto",
    "grSim_Packet.proto",
    "grSim_Replacement.proto",
    "messages_robocup_ssl_detection.proto",
    "messages_robocup_ssl_geometry.proto",
    "messages_robocup_ssl_refbox_log.proto",
    "messages_robocup_ssl_wrapper.proto",
    "referee.proto",
];

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let proto = root.join("proto");

    let mut compiler = protobuf_build::Compiler::new(&proto, &out);

    for file in PROTO_FILES {
        compiler.compile(file).unwrap();
    }
}
