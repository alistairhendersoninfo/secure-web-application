fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto = "../../proto/swap/v1/controller_agent.proto";
    println!("cargo:rerun-if-changed={}", proto);
    tonic_build::configure()
        .build_server(false)
        .compile(&[proto], &["../../proto"])?;
    Ok(())
}
