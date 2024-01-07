fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(&["proto/api/v1/glue.proto"], &["proto/api/v1/"])?;
    Ok(())
}
