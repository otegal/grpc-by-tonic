fn main() -> anyhow::Result<()> {
    tonic_build::compile_protos("helloworld.proto")?;
    Ok(())
}
