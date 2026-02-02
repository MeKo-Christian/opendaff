/*
 * -------------------------------------------------------------------------------------
 *
 *  OpenDAFF - A free, open source software package for directional audio data
 *  Copyright 2016-2018 Institute of Technical Acoustics (ITA), RWTH Aachen University
 *  OpenDAFF is distributed under the Apache License Version 2.0.
 *
 *  ------------------------------------------------------------------------------------
 *
 */

#ifndef IW_DAFF_RUST_WRAPPER
#define IW_DAFF_RUST_WRAPPER

#if defined WIN32
#ifdef DAFFRUST_EXPORTS
#define DAFFRUST_API __declspec(dllexport)
#else
#define DAFFRUST_API __declspec(dllimport)
#endif
#else
#define DAFFRUST_API
#endif

#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle types
typedef void* RustDAFFReaderHandle;
typedef void* RustDAFFContentHandle;

// Error handling
DAFFRUST_API const char* RustDAFF_GetLastError();

// Reader operations
DAFFRUST_API RustDAFFReaderHandle RustDAFF_Create();
DAFFRUST_API void RustDAFF_Destroy(RustDAFFReaderHandle handle);
DAFFRUST_API bool RustDAFF_OpenFile(RustDAFFReaderHandle handle, const char* filename);
DAFFRUST_API void RustDAFF_Close(RustDAFFReaderHandle handle);
DAFFRUST_API bool RustDAFF_IsValid(RustDAFFReaderHandle handle);

// File properties
DAFFRUST_API int RustDAFF_GetContentType(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_GetQuantization(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_GetNumChannels(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_GetNumRecords(RustDAFFReaderHandle handle);
DAFFRUST_API float RustDAFF_GetAlphaResolution(RustDAFFReaderHandle handle);
DAFFRUST_API float RustDAFF_GetBetaResolution(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_GetAlphaPoints(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_GetBetaPoints(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_GetOrientationYPR(RustDAFFReaderHandle handle, float* yaw, float* pitch, float* roll);

// Metadata operations
DAFFRUST_API bool RustDAFF_HasMetadata(RustDAFFReaderHandle handle, const char* key);
DAFFRUST_API const char* RustDAFF_GetMetadataString(RustDAFFReaderHandle handle, const char* key);
DAFFRUST_API bool RustDAFF_GetMetadataFloat(RustDAFFReaderHandle handle, const char* key, float* value);
DAFFRUST_API bool RustDAFF_GetMetadataBool(RustDAFFReaderHandle handle, const char* key, bool* value);

// Content access - Impulse Response (IR)
DAFFRUST_API RustDAFFContentHandle RustDAFF_GetContentIR(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_ContentIR_GetFilterLength(RustDAFFContentHandle content);
DAFFRUST_API int RustDAFF_ContentIR_GetSamplerate(RustDAFFContentHandle content);
DAFFRUST_API int RustDAFF_ContentIR_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta);
DAFFRUST_API bool RustDAFF_ContentIR_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha,
													 double* beta);
DAFFRUST_API bool RustDAFF_ContentIR_GetFilterCoeffs(RustDAFFContentHandle content, int recordIndex, int channel,
													 float* coeffs, int bufferSize);

// Content access - Magnitude Spectrum (MS)
DAFFRUST_API RustDAFFContentHandle RustDAFF_GetContentMS(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_ContentMS_GetNumFrequencies(RustDAFFContentHandle content);
DAFFRUST_API int RustDAFF_ContentMS_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta);
DAFFRUST_API bool RustDAFF_ContentMS_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha,
													 double* beta);
DAFFRUST_API bool RustDAFF_ContentMS_GetMagnitudes(RustDAFFContentHandle content, int recordIndex, int channel,
												   float* magnitudes, int bufferSize);

// Content access - Phase Spectrum (PS)
DAFFRUST_API RustDAFFContentHandle RustDAFF_GetContentPS(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_ContentPS_GetNumFrequencies(RustDAFFContentHandle content);
DAFFRUST_API int RustDAFF_ContentPS_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta);
DAFFRUST_API bool RustDAFF_ContentPS_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha,
													 double* beta);
DAFFRUST_API bool RustDAFF_ContentPS_GetPhases(RustDAFFContentHandle content, int recordIndex, int channel,
											   float* phases, int bufferSize);

// Content access - Magnitude-Phase Spectrum (MPS)
DAFFRUST_API RustDAFFContentHandle RustDAFF_GetContentMPS(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_ContentMPS_GetNumFrequencies(RustDAFFContentHandle content);
DAFFRUST_API int RustDAFF_ContentMPS_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta);
DAFFRUST_API bool RustDAFF_ContentMPS_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha,
													  double* beta);
DAFFRUST_API bool RustDAFF_ContentMPS_GetCoefficients(RustDAFFContentHandle content, int recordIndex, int channel,
													  float* magnitudes, float* phases, int bufferSize);

// Content access - DFT
DAFFRUST_API RustDAFFContentHandle RustDAFF_GetContentDFT(RustDAFFReaderHandle handle);
DAFFRUST_API int RustDAFF_ContentDFT_GetNumDFTCoeffs(RustDAFFContentHandle content);
DAFFRUST_API bool RustDAFF_ContentDFT_IsSymmetric(RustDAFFContentHandle content);
DAFFRUST_API int RustDAFF_ContentDFT_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta);
DAFFRUST_API bool RustDAFF_ContentDFT_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha,
													  double* beta);
DAFFRUST_API bool RustDAFF_ContentDFT_GetDFTCoeffs(RustDAFFContentHandle content, int recordIndex, int channel,
												   float* coeffs, int bufferSize);

#ifdef __cplusplus
}
#endif

#endif  // IW_DAFF_RUST_WRAPPER
