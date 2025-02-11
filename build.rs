use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .cargo_out_dir("protos")
        .input("src/protos/example.proto")
        .include("src/protos")
        .run_from_script();
}
