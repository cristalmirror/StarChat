fn main() -> Result<(), Box<dny std::error::Error>> {
    tonic_build::compile_protos("proto/replication.proto")?;
    Ok(())
}
