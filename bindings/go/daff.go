// Package daff provides Go bindings for the OpenDAFF library.
//
// OpenDAFF is a free and open-source software package for directional audio content
// like directivities of sound sources (loudspeakers, musical instruments) and sound
// receivers (microphones, HRTFs/HRIRs). It enables exchange, representation, and
// efficient storage of directional data in the DAFF file format (*.daff).
//
// Example usage:
//
//	reader, err := daff.NewReader()
//	if err != nil {
//	    log.Fatal(err)
//	}
//	defer reader.Close()
//
//	if err := reader.OpenFile("example.daff"); err != nil {
//	    log.Fatal(err)
//	}
//
//	contentType := reader.GetContentType()
//	if contentType == daff.ContentTypeIR {
//	    ir, err := reader.GetContentIR()
//	    if err != nil {
//	        log.Fatal(err)
//	    }
//	    // Use impulse response data...
//	}
package daff

/*
#cgo CXXFLAGS: -I../../include -std=c++11
#cgo LDFLAGS: -L../../build -lDAFF -lstdc++
#include "daff_go_wrapper.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"runtime"
	"unsafe"
)

// ContentType represents the type of data stored in a DAFF file
type ContentType int

const (
	ContentTypeIR  ContentType = 1 // Impulse Response
	ContentTypeMS  ContentType = 2 // Magnitude Spectrum
	ContentTypePS  ContentType = 3 // Phase Spectrum
	ContentTypeMPS ContentType = 4 // Magnitude-Phase Spectrum
	ContentTypeDFT ContentType = 5 // DFT Coefficients
)

// String returns the string representation of the content type
func (c ContentType) String() string {
	switch c {
	case ContentTypeIR:
		return "ImpulseResponse"
	case ContentTypeMS:
		return "MagnitudeSpectrum"
	case ContentTypePS:
		return "PhaseSpectrum"
	case ContentTypeMPS:
		return "MagnitudePhaseSpectrum"
	case ContentTypeDFT:
		return "DFT"
	default:
		return "Unknown"
	}
}

// Quantization represents the data quantization type
type Quantization int

const (
	QuantizationInt16   Quantization = 1
	QuantizationInt24   Quantization = 2
	QuantizationFloat32 Quantization = 3
)

// String returns the string representation of the quantization type
func (q Quantization) String() string {
	switch q {
	case QuantizationInt16:
		return "Int16"
	case QuantizationInt24:
		return "Int24"
	case QuantizationFloat32:
		return "Float32"
	default:
		return "Unknown"
	}
}

// Reader provides access to DAFF files
type Reader struct {
	handle C.GoDAFFReaderHandle
}

// NewReader creates a new DAFF reader
func NewReader() (*Reader, error) {
	handle := C.GoDAFF_Create()
	if handle == nil {
		return nil, errors.New("failed to create DAFF reader")
	}

	reader := &Reader{handle: handle}
	runtime.SetFinalizer(reader, (*Reader).Close)
	return reader, nil
}

// Close releases resources associated with the reader
func (r *Reader) Close() error {
	if r.handle != nil {
		C.GoDAFF_Destroy(r.handle)
		r.handle = nil
	}
	return nil
}

// OpenFile opens a DAFF file for reading
func (r *Reader) OpenFile(filename string) error {
	cFilename := C.CString(filename)
	defer C.free(unsafe.Pointer(cFilename))

	if !C.GoDAFF_OpenFile(r.handle, cFilename) {
		return errors.New("failed to open file: " + filename)
	}
	return nil
}

// CloseFile closes the currently opened file
func (r *Reader) CloseFile() {
	C.GoDAFF_Close(r.handle)
}

// IsValid returns true if a file is currently opened and valid
func (r *Reader) IsValid() bool {
	return bool(C.GoDAFF_IsValid(r.handle))
}

// GetContentType returns the content type of the opened file
func (r *Reader) GetContentType() ContentType {
	return ContentType(C.GoDAFF_GetContentType(r.handle))
}

// GetQuantization returns the quantization type used in the file
func (r *Reader) GetQuantization() Quantization {
	return Quantization(C.GoDAFF_GetQuantization(r.handle))
}

// GetNumChannels returns the number of audio channels
func (r *Reader) GetNumChannels() int {
	return int(C.GoDAFF_GetNumChannels(r.handle))
}

// GetNumRecords returns the number of directional records
func (r *Reader) GetNumRecords() int {
	return int(C.GoDAFF_GetNumRecords(r.handle))
}

// GetAlphaResolution returns the angular resolution in alpha direction (degrees)
func (r *Reader) GetAlphaResolution() float32 {
	return float32(C.GoDAFF_GetAlphaResolution(r.handle))
}

// GetBetaResolution returns the angular resolution in beta direction (degrees)
func (r *Reader) GetBetaResolution() float32 {
	return float32(C.GoDAFF_GetBetaResolution(r.handle))
}

// GetAlphaPoints returns the number of sampling points in alpha direction
func (r *Reader) GetAlphaPoints() int {
	return int(C.GoDAFF_GetAlphaPoints(r.handle))
}

// GetBetaPoints returns the number of sampling points in beta direction
func (r *Reader) GetBetaPoints() int {
	return int(C.GoDAFF_GetBetaPoints(r.handle))
}

// GetOrientation returns the orientation as yaw, pitch, roll angles in degrees
func (r *Reader) GetOrientation() (yaw, pitch, roll float32, err error) {
	var cYaw, cPitch, cRoll C.float
	result := C.GoDAFF_GetOrientationYPR(r.handle, &cYaw, &cPitch, &cRoll)
	if result != 0 {
		return 0, 0, 0, errors.New("failed to get orientation")
	}
	return float32(cYaw), float32(cPitch), float32(cRoll), nil
}

// HasMetadata returns true if the specified metadata key exists
func (r *Reader) HasMetadata(key string) bool {
	cKey := C.CString(key)
	defer C.free(unsafe.Pointer(cKey))
	return bool(C.GoDAFF_HasMetadata(r.handle, cKey))
}

// GetMetadataString returns a string metadata value
func (r *Reader) GetMetadataString(key string) (string, error) {
	cKey := C.CString(key)
	defer C.free(unsafe.Pointer(cKey))

	cValue := C.GoDAFF_GetMetadataString(r.handle, cKey)
	if cValue == nil {
		return "", errors.New("metadata key not found: " + key)
	}
	return C.GoString(cValue), nil
}

// GetMetadataFloat returns a float metadata value
func (r *Reader) GetMetadataFloat(key string) (float32, error) {
	cKey := C.CString(key)
	defer C.free(unsafe.Pointer(cKey))

	var value C.float
	if !C.GoDAFF_GetMetadataFloat(r.handle, cKey, &value) {
		return 0, errors.New("metadata key not found: " + key)
	}
	return float32(value), nil
}

// GetMetadataBool returns a boolean metadata value
func (r *Reader) GetMetadataBool(key string) (bool, error) {
	cKey := C.CString(key)
	defer C.free(unsafe.Pointer(cKey))

	var value C.bool
	if !C.GoDAFF_GetMetadataBool(r.handle, cKey, &value) {
		return false, errors.New("metadata key not found: " + key)
	}
	return bool(value), nil
}

// ContentIR provides access to Impulse Response data
type ContentIR struct {
	handle C.GoDAFFContentHandle
}

// GetContentIR returns an impulse response content accessor
func (r *Reader) GetContentIR() (*ContentIR, error) {
	handle := C.GoDAFF_GetContentIR(r.handle)
	if handle == nil {
		return nil, errors.New("file does not contain impulse response data")
	}
	return &ContentIR{handle: handle}, nil
}

// GetFilterLength returns the length of the impulse response filters
func (c *ContentIR) GetFilterLength() int {
	return int(C.GoDAFF_ContentIR_GetFilterLength(c.handle))
}

// GetSamplerate returns the sample rate in Hz
func (c *ContentIR) GetSamplerate() int {
	return int(C.GoDAFF_ContentIR_GetSamplerate(c.handle))
}

// GetNearestNeighbour returns the record index for the given direction (phi, theta in radians)
func (c *ContentIR) GetNearestNeighbour(phi, theta float64) int {
	return int(C.GoDAFF_ContentIR_GetNearestNeighbour(c.handle, C.double(phi), C.double(theta)))
}

// GetRecordCoords returns the alpha and beta coordinates for the given record index
func (c *ContentIR) GetRecordCoords(recordIndex int) (alpha, beta float64, err error) {
	var cAlpha, cBeta C.double
	if !C.GoDAFF_ContentIR_GetRecordCoords(c.handle, C.int(recordIndex), &cAlpha, &cBeta) {
		return 0, 0, errors.New("failed to get record coordinates")
	}
	return float64(cAlpha), float64(cBeta), nil
}

// GetFilterCoeffs retrieves the filter coefficients for the specified record and channel
func (c *ContentIR) GetFilterCoeffs(recordIndex, channel int) ([]float32, error) {
	length := c.GetFilterLength()
	coeffs := make([]float32, length)

	if !C.GoDAFF_ContentIR_GetFilterCoeffs(c.handle, C.int(recordIndex), C.int(channel),
		(*C.float)(unsafe.Pointer(&coeffs[0])), C.int(length)) {
		return nil, errors.New("failed to get filter coefficients")
	}
	return coeffs, nil
}

// ContentMS provides access to Magnitude Spectrum data
type ContentMS struct {
	handle C.GoDAFFContentHandle
}

// GetContentMS returns a magnitude spectrum content accessor
func (r *Reader) GetContentMS() (*ContentMS, error) {
	handle := C.GoDAFF_GetContentMS(r.handle)
	if handle == nil {
		return nil, errors.New("file does not contain magnitude spectrum data")
	}
	return &ContentMS{handle: handle}, nil
}

// GetNumFrequencies returns the number of frequency bins
func (c *ContentMS) GetNumFrequencies() int {
	return int(C.GoDAFF_ContentMS_GetNumFrequencies(c.handle))
}

// GetNearestNeighbour returns the record index for the given direction
func (c *ContentMS) GetNearestNeighbour(phi, theta float64) int {
	return int(C.GoDAFF_ContentMS_GetNearestNeighbour(c.handle, C.double(phi), C.double(theta)))
}

// GetRecordCoords returns the coordinates for the given record index
func (c *ContentMS) GetRecordCoords(recordIndex int) (alpha, beta float64, err error) {
	var cAlpha, cBeta C.double
	if !C.GoDAFF_ContentMS_GetRecordCoords(c.handle, C.int(recordIndex), &cAlpha, &cBeta) {
		return 0, 0, errors.New("failed to get record coordinates")
	}
	return float64(cAlpha), float64(cBeta), nil
}

// GetMagnitudes retrieves magnitude values for the specified record and channel
func (c *ContentMS) GetMagnitudes(recordIndex, channel int) ([]float32, error) {
	numFreqs := c.GetNumFrequencies()
	magnitudes := make([]float32, numFreqs)

	if !C.GoDAFF_ContentMS_GetMagnitudes(c.handle, C.int(recordIndex), C.int(channel),
		(*C.float)(unsafe.Pointer(&magnitudes[0])), C.int(numFreqs)) {
		return nil, errors.New("failed to get magnitudes")
	}
	return magnitudes, nil
}

// ContentPS provides access to Phase Spectrum data
type ContentPS struct {
	handle C.GoDAFFContentHandle
}

// GetContentPS returns a phase spectrum content accessor
func (r *Reader) GetContentPS() (*ContentPS, error) {
	handle := C.GoDAFF_GetContentPS(r.handle)
	if handle == nil {
		return nil, errors.New("file does not contain phase spectrum data")
	}
	return &ContentPS{handle: handle}, nil
}

// GetNumFrequencies returns the number of frequency bins
func (c *ContentPS) GetNumFrequencies() int {
	return int(C.GoDAFF_ContentPS_GetNumFrequencies(c.handle))
}

// GetNearestNeighbour returns the record index for the given direction
func (c *ContentPS) GetNearestNeighbour(phi, theta float64) int {
	return int(C.GoDAFF_ContentPS_GetNearestNeighbour(c.handle, C.double(phi), C.double(theta)))
}

// GetRecordCoords returns the coordinates for the given record index
func (c *ContentPS) GetRecordCoords(recordIndex int) (alpha, beta float64, err error) {
	var cAlpha, cBeta C.double
	if !C.GoDAFF_ContentPS_GetRecordCoords(c.handle, C.int(recordIndex), &cAlpha, &cBeta) {
		return 0, 0, errors.New("failed to get record coordinates")
	}
	return float64(cAlpha), float64(cBeta), nil
}

// GetPhases retrieves phase values for the specified record and channel
func (c *ContentPS) GetPhases(recordIndex, channel int) ([]float32, error) {
	numFreqs := c.GetNumFrequencies()
	phases := make([]float32, numFreqs)

	if !C.GoDAFF_ContentPS_GetPhases(c.handle, C.int(recordIndex), C.int(channel),
		(*C.float)(unsafe.Pointer(&phases[0])), C.int(numFreqs)) {
		return nil, errors.New("failed to get phases")
	}
	return phases, nil
}

// ContentMPS provides access to Magnitude-Phase Spectrum data
type ContentMPS struct {
	handle C.GoDAFFContentHandle
}

// GetContentMPS returns a magnitude-phase spectrum content accessor
func (r *Reader) GetContentMPS() (*ContentMPS, error) {
	handle := C.GoDAFF_GetContentMPS(r.handle)
	if handle == nil {
		return nil, errors.New("file does not contain magnitude-phase spectrum data")
	}
	return &ContentMPS{handle: handle}, nil
}

// GetNumFrequencies returns the number of frequency bins
func (c *ContentMPS) GetNumFrequencies() int {
	return int(C.GoDAFF_ContentMPS_GetNumFrequencies(c.handle))
}

// GetNearestNeighbour returns the record index for the given direction
func (c *ContentMPS) GetNearestNeighbour(phi, theta float64) int {
	return int(C.GoDAFF_ContentMPS_GetNearestNeighbour(c.handle, C.double(phi), C.double(theta)))
}

// GetRecordCoords returns the coordinates for the given record index
func (c *ContentMPS) GetRecordCoords(recordIndex int) (alpha, beta float64, err error) {
	var cAlpha, cBeta C.double
	if !C.GoDAFF_ContentMPS_GetRecordCoords(c.handle, C.int(recordIndex), &cAlpha, &cBeta) {
		return 0, 0, errors.New("failed to get record coordinates")
	}
	return float64(cAlpha), float64(cBeta), nil
}

// GetCoefficients retrieves both magnitude and phase values for the specified record and channel
func (c *ContentMPS) GetCoefficients(recordIndex, channel int) (magnitudes, phases []float32, err error) {
	numFreqs := c.GetNumFrequencies()
	magnitudes = make([]float32, numFreqs)
	phases = make([]float32, numFreqs)

	if !C.GoDAFF_ContentMPS_GetCoefficients(c.handle, C.int(recordIndex), C.int(channel),
		(*C.float)(unsafe.Pointer(&magnitudes[0])), (*C.float)(unsafe.Pointer(&phases[0])), C.int(numFreqs)) {
		return nil, nil, errors.New("failed to get coefficients")
	}
	return magnitudes, phases, nil
}

// ContentDFT provides access to DFT coefficient data
type ContentDFT struct {
	handle C.GoDAFFContentHandle
}

// GetContentDFT returns a DFT content accessor
func (r *Reader) GetContentDFT() (*ContentDFT, error) {
	handle := C.GoDAFF_GetContentDFT(r.handle)
	if handle == nil {
		return nil, errors.New("file does not contain DFT data")
	}
	return &ContentDFT{handle: handle}, nil
}

// GetNumDFTCoeffs returns the number of DFT coefficients
func (c *ContentDFT) GetNumDFTCoeffs() int {
	return int(C.GoDAFF_ContentDFT_GetNumDFTCoeffs(c.handle))
}

// IsSymmetric returns true if the DFT data is symmetric
func (c *ContentDFT) IsSymmetric() bool {
	return bool(C.GoDAFF_ContentDFT_IsSymmetric(c.handle))
}

// GetNearestNeighbour returns the record index for the given direction
func (c *ContentDFT) GetNearestNeighbour(phi, theta float64) int {
	return int(C.GoDAFF_ContentDFT_GetNearestNeighbour(c.handle, C.double(phi), C.double(theta)))
}

// GetRecordCoords returns the coordinates for the given record index
func (c *ContentDFT) GetRecordCoords(recordIndex int) (alpha, beta float64, err error) {
	var cAlpha, cBeta C.double
	if !C.GoDAFF_ContentDFT_GetRecordCoords(c.handle, C.int(recordIndex), &cAlpha, &cBeta) {
		return 0, 0, errors.New("failed to get record coordinates")
	}
	return float64(cAlpha), float64(cBeta), nil
}

// GetDFTCoeffs retrieves DFT coefficients (interleaved real/imaginary) for the specified record and channel
func (c *ContentDFT) GetDFTCoeffs(recordIndex, channel int) ([]float32, error) {
	numCoeffs := c.GetNumDFTCoeffs()
	coeffs := make([]float32, numCoeffs*2) // Complex values: real, imag interleaved

	if !C.GoDAFF_ContentDFT_GetDFTCoeffs(c.handle, C.int(recordIndex), C.int(channel),
		(*C.float)(unsafe.Pointer(&coeffs[0])), C.int(numCoeffs*2)) {
		return nil, errors.New("failed to get DFT coefficients")
	}
	return coeffs, nil
}
