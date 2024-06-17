use protobuf::Message;

mod protos;

fn main() -> anyhow::Result<()> {
    let file_path = std::env::temp_dir().join("repro.bin");
    let mut file = std::fs::File::create(file_path)?;
    let mut os = protobuf::CodedOutputStream::new(&mut file);

    let mut message = protos::example::MyMessage::new();
    message.my_strings.push("x".repeat(10000));

    message.write_length_delimited_to(&mut os)?;

    // If we got here, the issue is fixed or the reproduction conditions changed
    eprintln!("Failed to reproduce the issue");

    Ok(())
}
