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

#ifndef IW_DAFF_GO_WRAPPER
#define IW_DAFF_GO_WRAPPER

#if defined WIN32
#ifdef DAFFGO_EXPORTS
#define DAFFGO_API __declspec(dllexport)
#else
#define DAFFGO_API __declspec(dllimport)
#endif
#else
#define DAFFGO_API
#endif

#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque handle types
typedef void* GoDAFFReaderHandle;
typedef void* GoDAFFContentHandle;

// Error handling
DAFFGO_API const char* GoDAFF_GetLastError();

// Reader operations
DAFFGO_API GoDAFFReaderHandle GoDAFF_Create();
DAFFGO_API void GoDAFF_Destroy(GoDAFFReaderHandle handle);
DAFFGO_API bool GoDAFF_OpenFile(GoDAFFReaderHandle handle, const char* filename);
DAFFGO_API void GoDAFF_Close(GoDAFFReaderHandle handle);
DAFFGO_API bool GoDAFF_IsValid(GoDAFFReaderHandle handle);

// File properties
DAFFGO_API int GoDAFF_GetContentType(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_GetQuantization(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_GetNumChannels(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_GetNumRecords(GoDAFFReaderHandle handle);
DAFFGO_API float GoDAFF_GetAlphaResolution(GoDAFFReaderHandle handle);
DAFFGO_API float GoDAFF_GetBetaResolution(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_GetAlphaPoints(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_GetBetaPoints(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_GetOrientationYPR(GoDAFFReaderHandle handle, float* yaw, float* pitch, float* roll);

// Metadata operations
DAFFGO_API bool GoDAFF_HasMetadata(GoDAFFReaderHandle handle, const char* key);
DAFFGO_API const char* GoDAFF_GetMetadataString(GoDAFFReaderHandle handle, const char* key);
DAFFGO_API bool GoDAFF_GetMetadataFloat(GoDAFFReaderHandle handle, const char* key, float* value);
DAFFGO_API bool GoDAFF_GetMetadataBool(GoDAFFReaderHandle handle, const char* key, bool* value);

// Content access - Impulse Response (IR)
DAFFGO_API GoDAFFContentHandle GoDAFF_GetContentIR(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_ContentIR_GetFilterLength(GoDAFFContentHandle content);
DAFFGO_API int GoDAFF_ContentIR_GetSamplerate(GoDAFFContentHandle content);
DAFFGO_API int GoDAFF_ContentIR_GetNearestNeighbour(GoDAFFContentHandle content, double phi, double theta);
DAFFGO_API bool GoDAFF_ContentIR_GetRecordCoords(GoDAFFContentHandle content, int recordIndex, double* alpha, double* beta);
DAFFGO_API bool GoDAFF_ContentIR_GetFilterCoeffs(GoDAFFContentHandle content, int recordIndex, int channel, float* coeffs, int bufferSize);

// Content access - Magnitude Spectrum (MS)
DAFFGO_API GoDAFFContentHandle GoDAFF_GetContentMS(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_ContentMS_GetNumFrequencies(GoDAFFContentHandle content);
DAFFGO_API int GoDAFF_ContentMS_GetNearestNeighbour(GoDAFFContentHandle content, double phi, double theta);
DAFFGO_API bool GoDAFF_ContentMS_GetRecordCoords(GoDAFFContentHandle content, int recordIndex, double* alpha, double* beta);
DAFFGO_API bool GoDAFF_ContentMS_GetMagnitudes(GoDAFFContentHandle content, int recordIndex, int channel, float* magnitudes, int bufferSize);

// Content access - Phase Spectrum (PS)
DAFFGO_API GoDAFFContentHandle GoDAFF_GetContentPS(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_ContentPS_GetNumFrequencies(GoDAFFContentHandle content);
DAFFGO_API int GoDAFF_ContentPS_GetNearestNeighbour(GoDAFFContentHandle content, double phi, double theta);
DAFFGO_API bool GoDAFF_ContentPS_GetRecordCoords(GoDAFFContentHandle content, int recordIndex, double* alpha, double* beta);
DAFFGO_API bool GoDAFF_ContentPS_GetPhases(GoDAFFContentHandle content, int recordIndex, int channel, float* phases, int bufferSize);

// Content access - Magnitude-Phase Spectrum (MPS)
DAFFGO_API GoDAFFContentHandle GoDAFF_GetContentMPS(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_ContentMPS_GetNumFrequencies(GoDAFFContentHandle content);
DAFFGO_API int GoDAFF_ContentMPS_GetNearestNeighbour(GoDAFFContentHandle content, double phi, double theta);
DAFFGO_API bool GoDAFF_ContentMPS_GetRecordCoords(GoDAFFContentHandle content, int recordIndex, double* alpha, double* beta);
DAFFGO_API bool GoDAFF_ContentMPS_GetCoefficients(GoDAFFContentHandle content, int recordIndex, int channel, float* magnitudes, float* phases, int bufferSize);

// Content access - DFT
DAFFGO_API GoDAFFContentHandle GoDAFF_GetContentDFT(GoDAFFReaderHandle handle);
DAFFGO_API int GoDAFF_ContentDFT_GetNumDFTCoeffs(GoDAFFContentHandle content);
DAFFGO_API bool GoDAFF_ContentDFT_IsSymmetric(GoDAFFContentHandle content);
DAFFGO_API int GoDAFF_ContentDFT_GetNearestNeighbour(GoDAFFContentHandle content, double phi, double theta);
DAFFGO_API bool GoDAFF_ContentDFT_GetRecordCoords(GoDAFFContentHandle content, int recordIndex, double* alpha, double* beta);
DAFFGO_API bool GoDAFF_ContentDFT_GetDFTCoeffs(GoDAFFContentHandle content, int recordIndex, int channel, float* coeffs, int bufferSize);

#ifdef __cplusplus
}
#endif

#endif // IW_DAFF_GO_WRAPPER
