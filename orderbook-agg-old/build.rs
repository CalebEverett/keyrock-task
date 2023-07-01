fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("proto/orderbook.proto")?;
    tonic_build::configure()
        .type_attribute(
            "booksummary.ExchangeType",
            "#[derive(strum::EnumIter, serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["proto/booksummary.proto"], &["proto"])?;
    Ok(())
}
