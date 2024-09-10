fn main() {
    protobuf_codegen::Codegen::new()
    .out_dir("src/")
    .inputs(&["src/hello.proto"])
    .include("src/")
    .run()
    .expect("Running protoc failed.");
}