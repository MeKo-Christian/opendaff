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

#include "daff_rust_wrapper.h"

#include <DAFF.h>

#include <cstring>
#include <string>
#include <vector>

// Thread-local storage for error messages
static thread_local std::string g_lastError;

void SetLastError(const std::string& error)
{
	g_lastError = error;
}

const char* RustDAFF_GetLastError()
{
	return g_lastError.c_str();
}

// Reader operations
RustDAFFReaderHandle RustDAFF_Create()
{
	try {
		DAFFReader* reader = DAFFReader::create();
		return static_cast<RustDAFFReaderHandle>(reader);
	} catch (const std::exception& e) {
		SetLastError(e.what());
		return nullptr;
	}
}

void RustDAFF_Destroy(RustDAFFReaderHandle handle)
{
	if (handle) {
		DAFFReader* reader = static_cast<DAFFReader*>(handle);
		delete reader;
	}
}

bool RustDAFF_OpenFile(RustDAFFReaderHandle handle, const char* filename)
{
	if (!handle || !filename) {
		SetLastError("Invalid handle or filename");
		return false;
	}
	try {
		DAFFReader* reader = static_cast<DAFFReader*>(handle);
		int result = reader->openFile(filename);
		if (result != DAFF_NO_ERROR) {
			SetLastError("Failed to open file: " + std::string(filename));
			return false;
		}
		return true;
	} catch (const std::exception& e) {
		SetLastError(e.what());
		return false;
	}
}

void RustDAFF_Close(RustDAFFReaderHandle handle)
{
	if (handle) {
		DAFFReader* reader = static_cast<DAFFReader*>(handle);
		reader->closeFile();
	}
}

bool RustDAFF_IsValid(RustDAFFReaderHandle handle)
{
	if (!handle)
		return false;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->isFileOpened();
}

// File properties
int RustDAFF_GetContentType(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getContentType();
}

int RustDAFF_GetQuantization(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getQuantization();
}

int RustDAFF_GetNumChannels(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getNumberOfChannels();
}

int RustDAFF_GetNumRecords(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getNumberOfRecords();
}

float RustDAFF_GetAlphaResolution(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1.0f;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getAlphaResolution();
}

float RustDAFF_GetBetaResolution(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1.0f;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getBetaResolution();
}

int RustDAFF_GetAlphaPoints(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getAlphaPoints();
}

int RustDAFF_GetBetaPoints(RustDAFFReaderHandle handle)
{
	if (!handle)
		return -1;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getProperties()->getBetaPoints();
}

int RustDAFF_GetOrientationYPR(RustDAFFReaderHandle handle, float* yaw, float* pitch, float* roll)
{
	if (!handle || !yaw || !pitch || !roll)
		return -1;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	DAFFOrientationYPR o;
	reader->getProperties()->getOrientation(o);
	*yaw = o.fYawAngleDeg;
	*pitch = o.fPitchAngleDeg;
	*roll = o.fRollAngleDeg;
	return 0;
}

// Metadata operations
bool RustDAFF_HasMetadata(RustDAFFReaderHandle handle, const char* key)
{
	if (!handle || !key)
		return false;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	return reader->getMetadata()->hasKey(key);
}

const char* RustDAFF_GetMetadataString(RustDAFFReaderHandle handle, const char* key)
{
	if (!handle || !key)
		return nullptr;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (!reader->getMetadata()->hasKey(key))
		return nullptr;
	static thread_local std::string value;
	value = reader->getMetadata()->getKeyString(key);
	return value.c_str();
}

bool RustDAFF_GetMetadataFloat(RustDAFFReaderHandle handle, const char* key, float* value)
{
	if (!handle || !key || !value)
		return false;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (!reader->getMetadata()->hasKey(key))
		return false;
	*value = static_cast<float>(reader->getMetadata()->getKeyFloat(key));
	return true;
}

bool RustDAFF_GetMetadataBool(RustDAFFReaderHandle handle, const char* key, bool* value)
{
	if (!handle || !key || !value)
		return false;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (!reader->getMetadata()->hasKey(key))
		return false;
	*value = reader->getMetadata()->getKeyBool(key);
	return true;
}

// Content access - Impulse Response (IR)
RustDAFFContentHandle RustDAFF_GetContentIR(RustDAFFReaderHandle handle)
{
	if (!handle)
		return nullptr;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (reader->getProperties()->getContentType() != DAFF_IMPULSE_RESPONSE)
		return nullptr;
	return static_cast<RustDAFFContentHandle>(dynamic_cast<DAFFContentIR*>(reader->getContent()));
}

int RustDAFF_ContentIR_GetFilterLength(RustDAFFContentHandle content)
{
	if (!content)
		return -1;
	DAFFContentIR* ir = static_cast<DAFFContentIR*>(content);
	return ir->getFilterLength();
}

int RustDAFF_ContentIR_GetSamplerate(RustDAFFContentHandle content)
{
	if (!content)
		return -1;
	DAFFContentIR* ir = static_cast<DAFFContentIR*>(content);
	return ir->getSamplerate();
}

int RustDAFF_ContentIR_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta)
{
	if (!content)
		return -1;
	DAFFContentIR* ir = static_cast<DAFFContentIR*>(content);
	int recordIndex;
	ir->getNearestNeighbour(DAFF_OBJECT_VIEW, static_cast<float>(phi), static_cast<float>(theta), recordIndex);
	return recordIndex;
}

bool RustDAFF_ContentIR_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha, double* beta)
{
	if (!content || !alpha || !beta)
		return false;
	DAFFContentIR* ir = static_cast<DAFFContentIR*>(content);
	float fAlpha, fBeta;
	ir->getRecordCoords(recordIndex, DAFF_DATA_VIEW, fAlpha, fBeta);
	*alpha = fAlpha;
	*beta = fBeta;
	return true;
}

bool RustDAFF_ContentIR_GetFilterCoeffs(RustDAFFContentHandle content, int recordIndex, int channel, float* coeffs,
										int bufferSize)
{
	if (!content || !coeffs)
		return false;
	DAFFContentIR* ir = static_cast<DAFFContentIR*>(content);
	if (bufferSize < ir->getFilterLength())
		return false;
	ir->getFilterCoeffs(recordIndex, channel, coeffs);
	return true;
}

// Content access - Magnitude Spectrum (MS)
RustDAFFContentHandle RustDAFF_GetContentMS(RustDAFFReaderHandle handle)
{
	if (!handle)
		return nullptr;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (reader->getProperties()->getContentType() != DAFF_MAGNITUDE_SPECTRUM)
		return nullptr;
	return static_cast<RustDAFFContentHandle>(dynamic_cast<DAFFContentMS*>(reader->getContent()));
}

int RustDAFF_ContentMS_GetNumFrequencies(RustDAFFContentHandle content)
{
	if (!content)
		return -1;
	DAFFContentMS* ms = static_cast<DAFFContentMS*>(content);
	return ms->getNumFrequencies();
}

int RustDAFF_ContentMS_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta)
{
	if (!content)
		return -1;
	DAFFContentMS* ms = static_cast<DAFFContentMS*>(content);
	int recordIndex;
	ms->getNearestNeighbour(DAFF_OBJECT_VIEW, static_cast<float>(phi), static_cast<float>(theta), recordIndex);
	return recordIndex;
}

bool RustDAFF_ContentMS_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha, double* beta)
{
	if (!content || !alpha || !beta)
		return false;
	DAFFContentMS* ms = static_cast<DAFFContentMS*>(content);
	float fAlpha, fBeta;
	ms->getRecordCoords(recordIndex, DAFF_DATA_VIEW, fAlpha, fBeta);
	*alpha = fAlpha;
	*beta = fBeta;
	return true;
}

bool RustDAFF_ContentMS_GetMagnitudes(RustDAFFContentHandle content, int recordIndex, int channel, float* magnitudes,
									  int bufferSize)
{
	if (!content || !magnitudes)
		return false;
	DAFFContentMS* ms = static_cast<DAFFContentMS*>(content);
	if (bufferSize < ms->getNumFrequencies())
		return false;
	ms->getMagnitudes(recordIndex, channel, magnitudes);
	return true;
}

// Content access - Phase Spectrum (PS)
RustDAFFContentHandle RustDAFF_GetContentPS(RustDAFFReaderHandle handle)
{
	if (!handle)
		return nullptr;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (reader->getProperties()->getContentType() != DAFF_PHASE_SPECTRUM)
		return nullptr;
	return static_cast<RustDAFFContentHandle>(dynamic_cast<DAFFContentPS*>(reader->getContent()));
}

int RustDAFF_ContentPS_GetNumFrequencies(RustDAFFContentHandle content)
{
	if (!content)
		return -1;
	DAFFContentPS* ps = static_cast<DAFFContentPS*>(content);
	return ps->getNumFrequencies();
}

int RustDAFF_ContentPS_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta)
{
	if (!content)
		return -1;
	DAFFContentPS* ps = static_cast<DAFFContentPS*>(content);
	int recordIndex;
	ps->getNearestNeighbour(DAFF_OBJECT_VIEW, static_cast<float>(phi), static_cast<float>(theta), recordIndex);
	return recordIndex;
}

bool RustDAFF_ContentPS_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha, double* beta)
{
	if (!content || !alpha || !beta)
		return false;
	DAFFContentPS* ps = static_cast<DAFFContentPS*>(content);
	float fAlpha, fBeta;
	ps->getRecordCoords(recordIndex, DAFF_DATA_VIEW, fAlpha, fBeta);
	*alpha = fAlpha;
	*beta = fBeta;
	return true;
}

bool RustDAFF_ContentPS_GetPhases(RustDAFFContentHandle content, int recordIndex, int channel, float* phases,
								  int bufferSize)
{
	if (!content || !phases)
		return false;
	DAFFContentPS* ps = static_cast<DAFFContentPS*>(content);
	if (bufferSize < ps->getNumFrequencies())
		return false;
	ps->getPhases(recordIndex, channel, phases);
	return true;
}

// Content access - Magnitude-Phase Spectrum (MPS)
RustDAFFContentHandle RustDAFF_GetContentMPS(RustDAFFReaderHandle handle)
{
	if (!handle)
		return nullptr;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (reader->getProperties()->getContentType() != DAFF_MAGNITUDE_PHASE_SPECTRUM)
		return nullptr;
	return static_cast<RustDAFFContentHandle>(dynamic_cast<DAFFContentMPS*>(reader->getContent()));
}

int RustDAFF_ContentMPS_GetNumFrequencies(RustDAFFContentHandle content)
{
	if (!content)
		return -1;
	DAFFContentMPS* mps = static_cast<DAFFContentMPS*>(content);
	return mps->getNumFrequencies();
}

int RustDAFF_ContentMPS_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta)
{
	if (!content)
		return -1;
	DAFFContentMPS* mps = static_cast<DAFFContentMPS*>(content);
	int recordIndex;
	mps->getNearestNeighbour(DAFF_OBJECT_VIEW, static_cast<float>(phi), static_cast<float>(theta), recordIndex);
	return recordIndex;
}

bool RustDAFF_ContentMPS_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha, double* beta)
{
	if (!content || !alpha || !beta)
		return false;
	DAFFContentMPS* mps = static_cast<DAFFContentMPS*>(content);
	float fAlpha, fBeta;
	mps->getRecordCoords(recordIndex, DAFF_DATA_VIEW, fAlpha, fBeta);
	*alpha = fAlpha;
	*beta = fBeta;
	return true;
}

bool RustDAFF_ContentMPS_GetCoefficients(RustDAFFContentHandle content, int recordIndex, int channel, float* magnitudes,
										 float* phases, int bufferSize)
{
	if (!content || !magnitudes || !phases)
		return false;
	DAFFContentMPS* mps = static_cast<DAFFContentMPS*>(content);
	int numFreqs = mps->getNumFrequencies();
	if (bufferSize < numFreqs)
		return false;

	// Allocate temporary buffer for interleaved data (Mag[0], Ph[0], Mag[1], Ph[1], ...)
	std::vector<float> interleaved(numFreqs * 2);
	int result = mps->getCoefficientsMP(recordIndex, channel, interleaved.data());
	if (result != DAFF_NO_ERROR)
		return false;

	// De-interleave into separate magnitude and phase arrays
	for (int i = 0; i < numFreqs; i++) {
		magnitudes[i] = interleaved[i * 2];
		phases[i] = interleaved[i * 2 + 1];
	}

	return true;
}

// Content access - DFT
RustDAFFContentHandle RustDAFF_GetContentDFT(RustDAFFReaderHandle handle)
{
	if (!handle)
		return nullptr;
	DAFFReader* reader = static_cast<DAFFReader*>(handle);
	if (reader->getProperties()->getContentType() != DAFF_DFT_SPECTRUM)
		return nullptr;
	return static_cast<RustDAFFContentHandle>(dynamic_cast<DAFFContentDFT*>(reader->getContent()));
}

int RustDAFF_ContentDFT_GetNumDFTCoeffs(RustDAFFContentHandle content)
{
	if (!content)
		return -1;
	DAFFContentDFT* dft = static_cast<DAFFContentDFT*>(content);
	return dft->getNumDFTCoeffs();
}

bool RustDAFF_ContentDFT_IsSymmetric(RustDAFFContentHandle content)
{
	if (!content)
		return false;
	DAFFContentDFT* dft = static_cast<DAFFContentDFT*>(content);
	return dft->isSymmetric();
}

int RustDAFF_ContentDFT_GetNearestNeighbour(RustDAFFContentHandle content, double phi, double theta)
{
	if (!content)
		return -1;
	DAFFContentDFT* dft = static_cast<DAFFContentDFT*>(content);
	int recordIndex;
	dft->getNearestNeighbour(DAFF_OBJECT_VIEW, static_cast<float>(phi), static_cast<float>(theta), recordIndex);
	return recordIndex;
}

bool RustDAFF_ContentDFT_GetRecordCoords(RustDAFFContentHandle content, int recordIndex, double* alpha, double* beta)
{
	if (!content || !alpha || !beta)
		return false;
	DAFFContentDFT* dft = static_cast<DAFFContentDFT*>(content);
	float fAlpha, fBeta;
	dft->getRecordCoords(recordIndex, DAFF_DATA_VIEW, fAlpha, fBeta);
	*alpha = fAlpha;
	*beta = fBeta;
	return true;
}

bool RustDAFF_ContentDFT_GetDFTCoeffs(RustDAFFContentHandle content, int recordIndex, int channel, float* coeffs,
									  int bufferSize)
{
	if (!content || !coeffs)
		return false;
	DAFFContentDFT* dft = static_cast<DAFFContentDFT*>(content);
	if (bufferSize < dft->getNumDFTCoeffs() * 2)
		return false; // DFT coeffs are complex (real, imag)
	dft->getDFTCoeffs(recordIndex, channel, coeffs);
	return true;
}
