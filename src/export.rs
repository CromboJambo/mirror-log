use polars::prelude::*;
use polars_sql::Database;
use std::fs::File;

/// Convert SQLite events table to a Polars DataFrame
pub fn sqlite_to_polars(db_path: &str) -> Result<DataFrame, PolarsError> {
    let conn = Database::open(db_path)?;
    let df = DataFrame::new(conn.execute("SELECT * FROM events", &[])?.to_dataframe()?);
    Ok(&mut df)
}

/// Export dataset to Parquet file
pub fn export_dataset(db_path: &str, output: &str) -> Result<(), PolarsError> {
    let mut df = sqlite_to_polars(db_path)?;

    let file = File::create(output)?;
    let writer = ParquetWriter::new(file);
    writer
        .with_compression(ParquetCompression::Snappy)
        .finish(&mut df)?;

    Ok(())
}
