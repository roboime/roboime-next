extern crate protobuf;

use std::os;
use std::io::File;

fn gen_protoc() {
    let req = parse_from_reader::<CodeGeneratorRequest>(&mut io::stdio::stdin() as &mut Reader).unwrap();
    let gen_options = GenOptions {
        dummy: false,
    };
    let result = gen(req.get_proto_file(), req.get_file_to_generate(), &gen_options);
    let mut resp = CodeGeneratorResponse::new();
    resp.set_file(result.iter().map(|file| {
        let mut r = CodeGeneratorResponse_File::new();
        r.set_name(file.name.to_string());
        r.set_content(str::from_utf8(file.content.as_slice()).unwrap().to_string());
        r
    }).collect());
    resp.write_to_writer(&mut io::stdout() as &mut Writer).unwrap();
}

fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("hello.rs")).unwrap();
    f.write_str("
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ").unwrap();
}
