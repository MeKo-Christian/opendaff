//! OpenDAFF Rust Bindings
//!
//! Rust bindings for the OpenDAFF library, providing idiomatic Rust access to directional
//! audio data in DAFF format.
//!
//! # Overview
//!
//! OpenDAFF is a software package for directional audio content like directivities of sound
//! sources (loudspeakers, musical instruments) and sound receivers (microphones, HRTFs/HRIRs).
//!
//! # Features
//!
//! - **Idiomatic Rust API** with proper error handling and resource management
//! - **Full content type support**: IR, MS, PS, MPS, and DFT
//! - **Zero-copy data access** where possible
//! - **Automatic resource cleanup** with RAII (Drop trait)
//! - **Type-safe** enums and structures
//!
//! # Quick Start
//!
//! ```no_run
//! use opendaff::{Reader, ContentType};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut reader = Reader::new()?;
//!     reader.open_file("path/to/file.daff")?;
//!
//!     let content_type = reader.content_type();
//!     println!("Content Type: {:?}", content_type);
//!     println!("Channels: {}", reader.num_channels());
//!     println!("Records: {}", reader.num_records());
//!
//!     match content_type {
//!         ContentType::ImpulseResponse => {
//!             let ir = reader.content_ir()?;
//!             println!("Filter Length: {}", ir.filter_length());
//!             println!("Sample Rate: {}", ir.samplerate());
//!
//!             // Get impulse response for front direction
//!             let record_idx = ir.nearest_neighbour(0.0, 0.0);
//!             let coeffs = ir.filter_coeffs(record_idx, 0)?;
//!             println!("Retrieved {} filter coefficients", coeffs.len());
//!         },
//!         _ => println!("Other content types..."),
//!     }
//!
//!     Ok(())
//! }
//! ```

mod ffi;

use std::error::Error as StdError;
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;

/// Result type for DAFF operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for DAFF operations
#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    fn from_last_error() -> Self {
        unsafe {
            let c_str = ffi::RustDAFF_GetLastError();
            if c_str.is_null() {
                Self::new("Unknown error")
            } else {
                let msg = CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned();
                Self::new(msg)
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DAFF error: {}", self.message)
    }
}

impl StdError for Error {}

/// Content types supported by DAFF files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ContentType {
    /// Impulse response
    ImpulseResponse = 1,
    /// Magnitude spectrum
    MagnitudeSpectrum = 2,
    /// Phase spectrum
    PhaseSpectrum = 3,
    /// Magnitude-phase spectrum
    MagnitudePhaseSpectrum = 4,
    /// DFT coefficients
    DftSpectrum = 5,
}

impl ContentType {
    fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(ContentType::ImpulseResponse),
            2 => Some(ContentType::MagnitudeSpectrum),
            3 => Some(ContentType::PhaseSpectrum),
            4 => Some(ContentType::MagnitudePhaseSpectrum),
            5 => Some(ContentType::DftSpectrum),
            _ => None,
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::ImpulseResponse => write!(f, "Impulse Response"),
            ContentType::MagnitudeSpectrum => write!(f, "Magnitude Spectrum"),
            ContentType::PhaseSpectrum => write!(f, "Phase Spectrum"),
            ContentType::MagnitudePhaseSpectrum => write!(f, "Magnitude-Phase Spectrum"),
            ContentType::DftSpectrum => write!(f, "DFT Spectrum"),
        }
    }
}

/// Quantization type for DAFF data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Quantization {
    /// 8-bit integer
    Int8 = 1,
    /// 16-bit integer
    Int16 = 2,
    /// 24-bit integer
    Int24 = 3,
    /// 32-bit integer
    Int32 = 4,
    /// 32-bit float
    Float32 = 5,
    /// 64-bit float
    Float64 = 6,
}

impl Quantization {
    fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Quantization::Int8),
            2 => Some(Quantization::Int16),
            3 => Some(Quantization::Int24),
            4 => Some(Quantization::Int32),
            5 => Some(Quantization::Float32),
            6 => Some(Quantization::Float64),
            _ => None,
        }
    }
}

/// Orientation in yaw-pitch-roll (degrees)
#[derive(Debug, Clone, Copy)]
pub struct Orientation {
    /// Yaw angle in degrees
    pub yaw: f32,
    /// Pitch angle in degrees
    pub pitch: f32,
    /// Roll angle in degrees
    pub roll: f32,
}

/// Main DAFF reader interface
pub struct Reader {
    handle: *mut ffi::RustDAFFReaderHandle,
}

impl Reader {
    /// Create a new DAFF reader
    pub fn new() -> Result<Self> {
        unsafe {
            let handle = ffi::RustDAFF_Create();
            if handle.is_null() {
                Err(Error::from_last_error())
            } else {
                Ok(Self { handle })
            }
        }
    }

    /// Open a DAFF file
    pub fn open_file(&mut self, filename: &str) -> Result<()> {
        let c_filename = CString::new(filename)
            .map_err(|_| Error::new("Invalid filename"))?;

        unsafe {
            if ffi::RustDAFF_OpenFile(self.handle, c_filename.as_ptr()) {
                Ok(())
            } else {
                Err(Error::from_last_error())
            }
        }
    }

    /// Close the currently open file
    pub fn close(&mut self) {
        unsafe {
            ffi::RustDAFF_Close(self.handle);
        }
    }

    /// Check if a file is currently open and valid
    pub fn is_valid(&self) -> bool {
        unsafe {
            ffi::RustDAFF_IsValid(self.handle)
        }
    }

    /// Get the content type of the open file
    pub fn content_type(&self) -> ContentType {
        unsafe {
            let ct = ffi::RustDAFF_GetContentType(self.handle);
            ContentType::from_i32(ct).unwrap_or(ContentType::ImpulseResponse)
        }
    }

    /// Get the quantization type
    pub fn quantization(&self) -> Option<Quantization> {
        unsafe {
            let q = ffi::RustDAFF_GetQuantization(self.handle);
            Quantization::from_i32(q)
        }
    }

    /// Get the number of channels
    pub fn num_channels(&self) -> i32 {
        unsafe {
            ffi::RustDAFF_GetNumChannels(self.handle)
        }
    }

    /// Get the number of records
    pub fn num_records(&self) -> i32 {
        unsafe {
            ffi::RustDAFF_GetNumRecords(self.handle)
        }
    }

    /// Get alpha resolution (azimuth)
    pub fn alpha_resolution(&self) -> f32 {
        unsafe {
            ffi::RustDAFF_GetAlphaResolution(self.handle)
        }
    }

    /// Get beta resolution (elevation)
    pub fn beta_resolution(&self) -> f32 {
        unsafe {
            ffi::RustDAFF_GetBetaResolution(self.handle)
        }
    }

    /// Get number of alpha points
    pub fn alpha_points(&self) -> i32 {
        unsafe {
            ffi::RustDAFF_GetAlphaPoints(self.handle)
        }
    }

    /// Get number of beta points
    pub fn beta_points(&self) -> i32 {
        unsafe {
            ffi::RustDAFF_GetBetaPoints(self.handle)
        }
    }

    /// Get orientation in yaw-pitch-roll
    pub fn orientation(&self) -> Result<Orientation> {
        let mut yaw = 0.0f32;
        let mut pitch = 0.0f32;
        let mut roll = 0.0f32;

        unsafe {
            if ffi::RustDAFF_GetOrientationYPR(self.handle, &mut yaw, &mut pitch, &mut roll) == 0 {
                Ok(Orientation { yaw, pitch, roll })
            } else {
                Err(Error::new("Failed to get orientation"))
            }
        }
    }

    /// Check if metadata key exists
    pub fn has_metadata(&self, key: &str) -> bool {
        let Ok(c_key) = CString::new(key) else {
            return false;
        };

        unsafe {
            ffi::RustDAFF_HasMetadata(self.handle, c_key.as_ptr())
        }
    }

    /// Get metadata value as string
    pub fn metadata_string(&self, key: &str) -> Result<String> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new("Invalid key"))?;

        unsafe {
            let c_str = ffi::RustDAFF_GetMetadataString(self.handle, c_key.as_ptr());
            if c_str.is_null() {
                Err(Error::new(format!("Metadata key '{}' not found", key)))
            } else {
                Ok(CStr::from_ptr(c_str)
                    .to_string_lossy()
                    .into_owned())
            }
        }
    }

    /// Get metadata value as float
    pub fn metadata_float(&self, key: &str) -> Result<f32> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new("Invalid key"))?;
        let mut value = 0.0f32;

        unsafe {
            if ffi::RustDAFF_GetMetadataFloat(self.handle, c_key.as_ptr(), &mut value) {
                Ok(value)
            } else {
                Err(Error::new(format!("Metadata key '{}' not found", key)))
            }
        }
    }

    /// Get metadata value as boolean
    pub fn metadata_bool(&self, key: &str) -> Result<bool> {
        let c_key = CString::new(key)
            .map_err(|_| Error::new("Invalid key"))?;
        let mut value = false;

        unsafe {
            if ffi::RustDAFF_GetMetadataBool(self.handle, c_key.as_ptr(), &mut value) {
                Ok(value)
            } else {
                Err(Error::new(format!("Metadata key '{}' not found", key)))
            }
        }
    }

    /// Get impulse response content
    pub fn content_ir(&self) -> Result<ContentIR<'_>> {
        unsafe {
            let content = ffi::RustDAFF_GetContentIR(self.handle);
            if content.is_null() {
                Err(Error::new("Not an IR content type"))
            } else {
                Ok(ContentIR {
                    handle: content,
                    _phantom: PhantomData,
                })
            }
        }
    }

    /// Get magnitude spectrum content
    pub fn content_ms(&self) -> Result<ContentMS<'_>> {
        unsafe {
            let content = ffi::RustDAFF_GetContentMS(self.handle);
            if content.is_null() {
                Err(Error::new("Not an MS content type"))
            } else {
                Ok(ContentMS {
                    handle: content,
                    _phantom: PhantomData,
                })
            }
        }
    }

    /// Get phase spectrum content
    pub fn content_ps(&self) -> Result<ContentPS<'_>> {
        unsafe {
            let content = ffi::RustDAFF_GetContentPS(self.handle);
            if content.is_null() {
                Err(Error::new("Not a PS content type"))
            } else {
                Ok(ContentPS {
                    handle: content,
                    _phantom: PhantomData,
                })
            }
        }
    }

    /// Get magnitude-phase spectrum content
    pub fn content_mps(&self) -> Result<ContentMPS<'_>> {
        unsafe {
            let content = ffi::RustDAFF_GetContentMPS(self.handle);
            if content.is_null() {
                Err(Error::new("Not an MPS content type"))
            } else {
                Ok(ContentMPS {
                    handle: content,
                    _phantom: PhantomData,
                })
            }
        }
    }

    /// Get DFT content
    pub fn content_dft(&self) -> Result<ContentDFT<'_>> {
        unsafe {
            let content = ffi::RustDAFF_GetContentDFT(self.handle);
            if content.is_null() {
                Err(Error::new("Not a DFT content type"))
            } else {
                Ok(ContentDFT {
                    handle: content,
                    _phantom: PhantomData,
                })
            }
        }
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            ffi::RustDAFF_Destroy(self.handle);
        }
    }
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

/// Impulse Response content
pub struct ContentIR<'a> {
    handle: *mut ffi::RustDAFFContentHandle,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ContentIR<'a> {
    /// Get the filter length (number of samples)
    pub fn filter_length(&self) -> i32 {
        unsafe { ffi::RustDAFF_ContentIR_GetFilterLength(self.handle) }
    }

    /// Get the sample rate in Hz
    pub fn samplerate(&self) -> i32 {
        unsafe { ffi::RustDAFF_ContentIR_GetSamplerate(self.handle) }
    }

    /// Find the nearest neighbour record for given angles
    ///
    /// # Arguments
    /// * `phi` - Azimuth angle in radians [0, 2π)
    /// * `theta` - Elevation angle in radians [-π/2, π/2]
    pub fn nearest_neighbour(&self, phi: f64, theta: f64) -> i32 {
        unsafe { ffi::RustDAFF_ContentIR_GetNearestNeighbour(self.handle, phi, theta) }
    }

    /// Get record coordinates
    ///
    /// Returns (alpha, beta) in data view coordinates
    pub fn record_coords(&self, record_index: i32) -> Result<(f64, f64)> {
        let mut alpha = 0.0;
        let mut beta = 0.0;

        unsafe {
            if ffi::RustDAFF_ContentIR_GetRecordCoords(
                self.handle,
                record_index,
                &mut alpha,
                &mut beta,
            ) {
                Ok((alpha, beta))
            } else {
                Err(Error::new("Failed to get record coordinates"))
            }
        }
    }

    /// Get filter coefficients for a given record and channel
    pub fn filter_coeffs(&self, record_index: i32, channel: i32) -> Result<Vec<f32>> {
        let length = self.filter_length() as usize;
        let mut coeffs = vec![0.0f32; length];

        unsafe {
            if ffi::RustDAFF_ContentIR_GetFilterCoeffs(
                self.handle,
                record_index,
                channel,
                coeffs.as_mut_ptr(),
                length as i32,
            ) {
                Ok(coeffs)
            } else {
                Err(Error::new("Failed to get filter coefficients"))
            }
        }
    }
}

/// Magnitude Spectrum content
pub struct ContentMS<'a> {
    handle: *mut ffi::RustDAFFContentHandle,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ContentMS<'a> {
    /// Get the number of frequencies
    pub fn num_frequencies(&self) -> i32 {
        unsafe { ffi::RustDAFF_ContentMS_GetNumFrequencies(self.handle) }
    }

    /// Find the nearest neighbour record for given angles
    pub fn nearest_neighbour(&self, phi: f64, theta: f64) -> i32 {
        unsafe { ffi::RustDAFF_ContentMS_GetNearestNeighbour(self.handle, phi, theta) }
    }

    /// Get record coordinates
    pub fn record_coords(&self, record_index: i32) -> Result<(f64, f64)> {
        let mut alpha = 0.0;
        let mut beta = 0.0;

        unsafe {
            if ffi::RustDAFF_ContentMS_GetRecordCoords(
                self.handle,
                record_index,
                &mut alpha,
                &mut beta,
            ) {
                Ok((alpha, beta))
            } else {
                Err(Error::new("Failed to get record coordinates"))
            }
        }
    }

    /// Get magnitude values for a given record and channel
    pub fn magnitudes(&self, record_index: i32, channel: i32) -> Result<Vec<f32>> {
        let length = self.num_frequencies() as usize;
        let mut magnitudes = vec![0.0f32; length];

        unsafe {
            if ffi::RustDAFF_ContentMS_GetMagnitudes(
                self.handle,
                record_index,
                channel,
                magnitudes.as_mut_ptr(),
                length as i32,
            ) {
                Ok(magnitudes)
            } else {
                Err(Error::new("Failed to get magnitudes"))
            }
        }
    }
}

/// Phase Spectrum content
pub struct ContentPS<'a> {
    handle: *mut ffi::RustDAFFContentHandle,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ContentPS<'a> {
    /// Get the number of frequencies
    pub fn num_frequencies(&self) -> i32 {
        unsafe { ffi::RustDAFF_ContentPS_GetNumFrequencies(self.handle) }
    }

    /// Find the nearest neighbour record for given angles
    pub fn nearest_neighbour(&self, phi: f64, theta: f64) -> i32 {
        unsafe { ffi::RustDAFF_ContentPS_GetNearestNeighbour(self.handle, phi, theta) }
    }

    /// Get record coordinates
    pub fn record_coords(&self, record_index: i32) -> Result<(f64, f64)> {
        let mut alpha = 0.0;
        let mut beta = 0.0;

        unsafe {
            if ffi::RustDAFF_ContentPS_GetRecordCoords(
                self.handle,
                record_index,
                &mut alpha,
                &mut beta,
            ) {
                Ok((alpha, beta))
            } else {
                Err(Error::new("Failed to get record coordinates"))
            }
        }
    }

    /// Get phase values for a given record and channel
    pub fn phases(&self, record_index: i32, channel: i32) -> Result<Vec<f32>> {
        let length = self.num_frequencies() as usize;
        let mut phases = vec![0.0f32; length];

        unsafe {
            if ffi::RustDAFF_ContentPS_GetPhases(
                self.handle,
                record_index,
                channel,
                phases.as_mut_ptr(),
                length as i32,
            ) {
                Ok(phases)
            } else {
                Err(Error::new("Failed to get phases"))
            }
        }
    }
}

/// Magnitude-Phase Spectrum content
pub struct ContentMPS<'a> {
    handle: *mut ffi::RustDAFFContentHandle,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ContentMPS<'a> {
    /// Get the number of frequencies
    pub fn num_frequencies(&self) -> i32 {
        unsafe { ffi::RustDAFF_ContentMPS_GetNumFrequencies(self.handle) }
    }

    /// Find the nearest neighbour record for given angles
    pub fn nearest_neighbour(&self, phi: f64, theta: f64) -> i32 {
        unsafe { ffi::RustDAFF_ContentMPS_GetNearestNeighbour(self.handle, phi, theta) }
    }

    /// Get record coordinates
    pub fn record_coords(&self, record_index: i32) -> Result<(f64, f64)> {
        let mut alpha = 0.0;
        let mut beta = 0.0;

        unsafe {
            if ffi::RustDAFF_ContentMPS_GetRecordCoords(
                self.handle,
                record_index,
                &mut alpha,
                &mut beta,
            ) {
                Ok((alpha, beta))
            } else {
                Err(Error::new("Failed to get record coordinates"))
            }
        }
    }

    /// Get magnitude and phase coefficients for a given record and channel
    ///
    /// Returns (magnitudes, phases) as separate vectors
    pub fn coefficients(&self, record_index: i32, channel: i32) -> Result<(Vec<f32>, Vec<f32>)> {
        let length = self.num_frequencies() as usize;
        let mut magnitudes = vec![0.0f32; length];
        let mut phases = vec![0.0f32; length];

        unsafe {
            if ffi::RustDAFF_ContentMPS_GetCoefficients(
                self.handle,
                record_index,
                channel,
                magnitudes.as_mut_ptr(),
                phases.as_mut_ptr(),
                length as i32,
            ) {
                Ok((magnitudes, phases))
            } else {
                Err(Error::new("Failed to get coefficients"))
            }
        }
    }
}

/// DFT Spectrum content
pub struct ContentDFT<'a> {
    handle: *mut ffi::RustDAFFContentHandle,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> ContentDFT<'a> {
    /// Get the number of DFT coefficients
    pub fn num_dft_coeffs(&self) -> i32 {
        unsafe { ffi::RustDAFF_ContentDFT_GetNumDFTCoeffs(self.handle) }
    }

    /// Check if DFT is symmetric
    pub fn is_symmetric(&self) -> bool {
        unsafe { ffi::RustDAFF_ContentDFT_IsSymmetric(self.handle) }
    }

    /// Find the nearest neighbour record for given angles
    pub fn nearest_neighbour(&self, phi: f64, theta: f64) -> i32 {
        unsafe { ffi::RustDAFF_ContentDFT_GetNearestNeighbour(self.handle, phi, theta) }
    }

    /// Get record coordinates
    pub fn record_coords(&self, record_index: i32) -> Result<(f64, f64)> {
        let mut alpha = 0.0;
        let mut beta = 0.0;

        unsafe {
            if ffi::RustDAFF_ContentDFT_GetRecordCoords(
                self.handle,
                record_index,
                &mut alpha,
                &mut beta,
            ) {
                Ok((alpha, beta))
            } else {
                Err(Error::new("Failed to get record coordinates"))
            }
        }
    }

    /// Get DFT coefficients for a given record and channel
    ///
    /// Returns interleaved real/imaginary values: [real0, imag0, real1, imag1, ...]
    pub fn dft_coeffs(&self, record_index: i32, channel: i32) -> Result<Vec<f32>> {
        let length = (self.num_dft_coeffs() * 2) as usize;
        let mut coeffs = vec![0.0f32; length];

        unsafe {
            if ffi::RustDAFF_ContentDFT_GetDFTCoeffs(
                self.handle,
                record_index,
                channel,
                coeffs.as_mut_ptr(),
                length as i32,
            ) {
                Ok(coeffs)
            } else {
                Err(Error::new("Failed to get DFT coefficients"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_creation() {
        let reader = Reader::new();
        assert!(reader.is_ok());
    }

    // Additional tests would require actual DAFF files
}
