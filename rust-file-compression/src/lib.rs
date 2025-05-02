use flate2::write::GzEncoder;
use flate2::Compression;

use std::fs::File;
use std::io;
use std::io::copy;
use std::io::BufReader;

pub fn compression(input_file: String, output_file: String) -> io::Result<()> {
    let mut input = BufReader::new(File::open(&input_file).unwrap());
    let output = File::create(&output_file)?;

    let mut encoder = GzEncoder::new(output, Compression::default());

    copy(&mut input, &mut encoder)?;
    let _ = encoder.finish().unwrap(); // Finish encoding and write final data
    println!("Compression complete. Compressed file: {}", output_file);

    Ok(())
}
