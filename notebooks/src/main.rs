use polars::prelude::*;

fn main() {
    let q = LazyCsvReader::new("./data/amazon_prime_titles.csv")
        .with_has_header(true)
        .finish()
        .unwrap();

    let df = q.collect().unwrap();

    let df_listed_in = df
        .clone()
        .lazy()
        .select([
            col("listed_in"),
            // col("listed_in")
            //     .apply(|s| { Ok(s.str()?.str_len_chars()) }, GetOutput::from_type(DataType::UInt32))
            //     .alias("listed_in_exploded"),
            // col("listed_in").cast(DataType::).alias("listed_in_exploded"),
        ])
        .collect()
        .unwrap();

    println!("{:?}", df_listed_in);
}
