//! Basic usage example for OpenDAFF Rust bindings

use opendaff::{ContentType, Reader};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get filename from command line args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <daff-file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Create reader and open file
    let mut reader = Reader::new()?;
    println!("Opening file: {}", filename);
    reader.open_file(filename)?;

    if !reader.is_valid() {
        eprintln!("File is not valid!");
        std::process::exit(1);
    }

    // Print file properties
    println!("\n=== File Properties ===");
    println!("Content Type: {}", reader.content_type());
    if let Some(quant) = reader.quantization() {
        println!("Quantization: {:?}", quant);
    }
    println!("Channels: {}", reader.num_channels());
    println!("Records: {}", reader.num_records());
    println!("Alpha Resolution: {:.4} rad", reader.alpha_resolution());
    println!("Beta Resolution: {:.4} rad", reader.beta_resolution());
    println!("Alpha Points: {}", reader.alpha_points());
    println!("Beta Points: {}", reader.beta_points());

    // Print orientation
    if let Ok(orientation) = reader.orientation() {
        println!("\n=== Orientation ===");
        println!("Yaw: {:.2}°", orientation.yaw);
        println!("Pitch: {:.2}°", orientation.pitch);
        println!("Roll: {:.2}°", orientation.roll);
    }

    // Print some metadata if available
    println!("\n=== Metadata ===");
    for key in &["Description", "Author", "Date", "Version"] {
        if reader.has_metadata(key) {
            if let Ok(value) = reader.metadata_string(key) {
                println!("{}: {}", key, value);
            }
        }
    }

    // Process based on content type
    match reader.content_type() {
        ContentType::ImpulseResponse => {
            println!("\n=== Impulse Response Content ===");
            let ir = reader.content_ir()?;
            println!("Filter Length: {} samples", ir.filter_length());
            println!("Sample Rate: {} Hz", ir.samplerate());
            println!(
                "Duration: {:.3} ms",
                (ir.filter_length() as f64 / ir.samplerate() as f64) * 1000.0
            );

            // Get impulse response for front direction (phi=0, theta=0)
            let record_idx = ir.nearest_neighbour(0.0, 0.0);
            println!("\nNearest record to (0°, 0°): {}", record_idx);

            let (alpha, beta) = ir.record_coords(record_idx)?;
            println!(
                "Record {} coordinates: alpha={:.4}, beta={:.4}",
                record_idx, alpha, beta
            );

            // Get filter coefficients for first channel
            let coeffs = ir.filter_coeffs(record_idx, 0)?;
            println!(
                "Retrieved {} filter coefficients for channel 0",
                coeffs.len()
            );
            println!(
                "First 5 coefficients: {:?}",
                &coeffs[..5.min(coeffs.len())]
            );

            // Find peak value
            let max_abs = coeffs.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
            println!("Peak amplitude: {:.6}", max_abs);
        }

        ContentType::MagnitudeSpectrum => {
            println!("\n=== Magnitude Spectrum Content ===");
            let ms = reader.content_ms()?;
            println!("Number of Frequencies: {}", ms.num_frequencies());

            let record_idx = ms.nearest_neighbour(0.0, 0.0);
            let magnitudes = ms.magnitudes(record_idx, 0)?;
            println!("Retrieved {} magnitude values", magnitudes.len());
            println!(
                "First 5 magnitudes: {:?}",
                &magnitudes[..5.min(magnitudes.len())]
            );
        }

        ContentType::PhaseSpectrum => {
            println!("\n=== Phase Spectrum Content ===");
            let ps = reader.content_ps()?;
            println!("Number of Frequencies: {}", ps.num_frequencies());

            let record_idx = ps.nearest_neighbour(0.0, 0.0);
            let phases = ps.phases(record_idx, 0)?;
            println!("Retrieved {} phase values", phases.len());
            println!("First 5 phases: {:?}", &phases[..5.min(phases.len())]);
        }

        ContentType::MagnitudePhaseSpectrum => {
            println!("\n=== Magnitude-Phase Spectrum Content ===");
            let mps = reader.content_mps()?;
            println!("Number of Frequencies: {}", mps.num_frequencies());

            let record_idx = mps.nearest_neighbour(0.0, 0.0);
            let (magnitudes, phases) = mps.coefficients(record_idx, 0)?;
            println!(
                "Retrieved {} magnitude and {} phase values",
                magnitudes.len(),
                phases.len()
            );
            println!(
                "First 5 magnitudes: {:?}",
                &magnitudes[..5.min(magnitudes.len())]
            );
            println!("First 5 phases: {:?}", &phases[..5.min(phases.len())]);
        }

        ContentType::DftSpectrum => {
            println!("\n=== DFT Spectrum Content ===");
            let dft = reader.content_dft()?;
            println!("Number of DFT Coefficients: {}", dft.num_dft_coeffs());
            println!("Is Symmetric: {}", dft.is_symmetric());

            let record_idx = dft.nearest_neighbour(0.0, 0.0);
            let coeffs = dft.dft_coeffs(record_idx, 0)?;
            println!(
                "Retrieved {} DFT coefficients (interleaved real/imag)",
                coeffs.len()
            );
            println!("First complex value: {:.6} + {:.6}i", coeffs[0], coeffs[1]);
        }
    }

    println!("\n=== Done ===");
    Ok(())
}
