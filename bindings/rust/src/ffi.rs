//! Raw FFI bindings to the OpenDAFF C wrapper library.
//!
//! This module contains unsafe FFI declarations. Use the safe wrappers in the parent module instead.

use std::os::raw::{c_char, c_double, c_float, c_int};

#[repr(C)]
pub struct RustDAFFReaderHandle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RustDAFFContentHandle {
    _private: [u8; 0],
}

extern "C" {
    // Error handling
    pub fn RustDAFF_GetLastError() -> *const c_char;

    // Reader operations
    pub fn RustDAFF_Create() -> *mut RustDAFFReaderHandle;
    pub fn RustDAFF_Destroy(handle: *mut RustDAFFReaderHandle);
    pub fn RustDAFF_OpenFile(handle: *mut RustDAFFReaderHandle, filename: *const c_char) -> bool;
    pub fn RustDAFF_Close(handle: *mut RustDAFFReaderHandle);
    pub fn RustDAFF_IsValid(handle: *const RustDAFFReaderHandle) -> bool;

    // File properties
    pub fn RustDAFF_GetContentType(handle: *const RustDAFFReaderHandle) -> c_int;
    pub fn RustDAFF_GetQuantization(handle: *const RustDAFFReaderHandle) -> c_int;
    pub fn RustDAFF_GetNumChannels(handle: *const RustDAFFReaderHandle) -> c_int;
    pub fn RustDAFF_GetNumRecords(handle: *const RustDAFFReaderHandle) -> c_int;
    pub fn RustDAFF_GetAlphaResolution(handle: *const RustDAFFReaderHandle) -> c_float;
    pub fn RustDAFF_GetBetaResolution(handle: *const RustDAFFReaderHandle) -> c_float;
    pub fn RustDAFF_GetAlphaPoints(handle: *const RustDAFFReaderHandle) -> c_int;
    pub fn RustDAFF_GetBetaPoints(handle: *const RustDAFFReaderHandle) -> c_int;
    pub fn RustDAFF_GetOrientationYPR(
        handle: *const RustDAFFReaderHandle,
        yaw: *mut c_float,
        pitch: *mut c_float,
        roll: *mut c_float,
    ) -> c_int;

    // Metadata operations
    pub fn RustDAFF_HasMetadata(handle: *const RustDAFFReaderHandle, key: *const c_char) -> bool;
    pub fn RustDAFF_GetMetadataString(
        handle: *const RustDAFFReaderHandle,
        key: *const c_char,
    ) -> *const c_char;
    pub fn RustDAFF_GetMetadataFloat(
        handle: *const RustDAFFReaderHandle,
        key: *const c_char,
        value: *mut c_float,
    ) -> bool;
    pub fn RustDAFF_GetMetadataBool(
        handle: *const RustDAFFReaderHandle,
        key: *const c_char,
        value: *mut bool,
    ) -> bool;

    // Content access - Impulse Response (IR)
    pub fn RustDAFF_GetContentIR(
        handle: *const RustDAFFReaderHandle,
    ) -> *mut RustDAFFContentHandle;
    pub fn RustDAFF_ContentIR_GetFilterLength(content: *const RustDAFFContentHandle) -> c_int;
    pub fn RustDAFF_ContentIR_GetSamplerate(content: *const RustDAFFContentHandle) -> c_int;
    pub fn RustDAFF_ContentIR_GetNearestNeighbour(
        content: *const RustDAFFContentHandle,
        phi: c_double,
        theta: c_double,
    ) -> c_int;
    pub fn RustDAFF_ContentIR_GetRecordCoords(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        alpha: *mut c_double,
        beta: *mut c_double,
    ) -> bool;
    pub fn RustDAFF_ContentIR_GetFilterCoeffs(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        channel: c_int,
        coeffs: *mut c_float,
        buffer_size: c_int,
    ) -> bool;

    // Content access - Magnitude Spectrum (MS)
    pub fn RustDAFF_GetContentMS(
        handle: *const RustDAFFReaderHandle,
    ) -> *mut RustDAFFContentHandle;
    pub fn RustDAFF_ContentMS_GetNumFrequencies(content: *const RustDAFFContentHandle) -> c_int;
    pub fn RustDAFF_ContentMS_GetNearestNeighbour(
        content: *const RustDAFFContentHandle,
        phi: c_double,
        theta: c_double,
    ) -> c_int;
    pub fn RustDAFF_ContentMS_GetRecordCoords(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        alpha: *mut c_double,
        beta: *mut c_double,
    ) -> bool;
    pub fn RustDAFF_ContentMS_GetMagnitudes(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        channel: c_int,
        magnitudes: *mut c_float,
        buffer_size: c_int,
    ) -> bool;

    // Content access - Phase Spectrum (PS)
    pub fn RustDAFF_GetContentPS(
        handle: *const RustDAFFReaderHandle,
    ) -> *mut RustDAFFContentHandle;
    pub fn RustDAFF_ContentPS_GetNumFrequencies(content: *const RustDAFFContentHandle) -> c_int;
    pub fn RustDAFF_ContentPS_GetNearestNeighbour(
        content: *const RustDAFFContentHandle,
        phi: c_double,
        theta: c_double,
    ) -> c_int;
    pub fn RustDAFF_ContentPS_GetRecordCoords(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        alpha: *mut c_double,
        beta: *mut c_double,
    ) -> bool;
    pub fn RustDAFF_ContentPS_GetPhases(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        channel: c_int,
        phases: *mut c_float,
        buffer_size: c_int,
    ) -> bool;

    // Content access - Magnitude-Phase Spectrum (MPS)
    pub fn RustDAFF_GetContentMPS(
        handle: *const RustDAFFReaderHandle,
    ) -> *mut RustDAFFContentHandle;
    pub fn RustDAFF_ContentMPS_GetNumFrequencies(content: *const RustDAFFContentHandle) -> c_int;
    pub fn RustDAFF_ContentMPS_GetNearestNeighbour(
        content: *const RustDAFFContentHandle,
        phi: c_double,
        theta: c_double,
    ) -> c_int;
    pub fn RustDAFF_ContentMPS_GetRecordCoords(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        alpha: *mut c_double,
        beta: *mut c_double,
    ) -> bool;
    pub fn RustDAFF_ContentMPS_GetCoefficients(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        channel: c_int,
        magnitudes: *mut c_float,
        phases: *mut c_float,
        buffer_size: c_int,
    ) -> bool;

    // Content access - DFT
    pub fn RustDAFF_GetContentDFT(
        handle: *const RustDAFFReaderHandle,
    ) -> *mut RustDAFFContentHandle;
    pub fn RustDAFF_ContentDFT_GetNumDFTCoeffs(content: *const RustDAFFContentHandle) -> c_int;
    pub fn RustDAFF_ContentDFT_IsSymmetric(content: *const RustDAFFContentHandle) -> bool;
    pub fn RustDAFF_ContentDFT_GetNearestNeighbour(
        content: *const RustDAFFContentHandle,
        phi: c_double,
        theta: c_double,
    ) -> c_int;
    pub fn RustDAFF_ContentDFT_GetRecordCoords(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        alpha: *mut c_double,
        beta: *mut c_double,
    ) -> bool;
    pub fn RustDAFF_ContentDFT_GetDFTCoeffs(
        content: *const RustDAFFContentHandle,
        record_index: c_int,
        channel: c_int,
        coeffs: *mut c_float,
        buffer_size: c_int,
    ) -> bool;
}
