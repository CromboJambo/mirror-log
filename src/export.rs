use polars::parquet;

pub fn export_dataset(db_path: &str, output: &str) -> Result<()> {
    let df = sqlite_to_polars(db_path)?;
    df.write_parquet(output, ParquetCompression::Snappy)?;
    Ok(())
}
