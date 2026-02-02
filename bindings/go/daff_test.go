package daff_test

import (
	"fmt"
	"testing"

	"github.com/MeKo-Tech/opendaff-go"
)

func Example() {
	// Create a new reader
	reader, err := daff.NewReader()
	if err != nil {
		fmt.Printf("Error creating reader: %v\n", err)
		return
	}
	defer reader.Close()

	// Open a DAFF file
	if err := reader.OpenFile("example.daff"); err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		return
	}
	defer reader.CloseFile()

	// Get file properties
	contentType := reader.GetContentType()
	numChannels := reader.GetNumChannels()
	numRecords := reader.GetNumRecords()

	fmt.Printf("Content Type: %s\n", contentType)
	fmt.Printf("Channels: %d\n", numChannels)
	fmt.Printf("Records: %d\n", numRecords)

	// Handle different content types
	switch contentType {
	case daff.ContentTypeIR:
		ir, err := reader.GetContentIR()
		if err != nil {
			fmt.Printf("Error getting IR content: %v\n", err)
			return
		}

		filterLength := ir.GetFilterLength()
		samplerate := ir.GetSamplerate()
		fmt.Printf("Filter Length: %d samples\n", filterLength)
		fmt.Printf("Sample Rate: %d Hz\n", samplerate)

		// Get nearest neighbour for front direction (phi=0, theta=0)
		recordIndex := ir.GetNearestNeighbour(0, 0)
		fmt.Printf("Nearest record for front: %d\n", recordIndex)

		// Get filter coefficients
		coeffs, err := ir.GetFilterCoeffs(recordIndex, 0)
		if err != nil {
			fmt.Printf("Error getting coefficients: %v\n", err)
			return
		}
		fmt.Printf("First 5 coefficients: %v\n", coeffs[:5])

	case daff.ContentTypeMS:
		ms, err := reader.GetContentMS()
		if err != nil {
			fmt.Printf("Error getting MS content: %v\n", err)
			return
		}

		numFreqs := ms.GetNumFrequencies()
		fmt.Printf("Number of frequencies: %d\n", numFreqs)

		// Get magnitude spectrum for front direction
		recordIndex := ms.GetNearestNeighbour(0, 0)
		magnitudes, err := ms.GetMagnitudes(recordIndex, 0)
		if err != nil {
			fmt.Printf("Error getting magnitudes: %v\n", err)
			return
		}
		fmt.Printf("First 5 magnitudes: %v\n", magnitudes[:5])

	default:
		fmt.Printf("Unsupported content type: %s\n", contentType)
	}
}

func TestReaderCreation(t *testing.T) {
	reader, err := daff.NewReader()
	if err != nil {
		t.Fatalf("Failed to create reader: %v", err)
	}
	defer reader.Close()

	if reader == nil {
		t.Fatal("Reader is nil")
	}
}

func TestContentTypeString(t *testing.T) {
	tests := []struct {
		contentType daff.ContentType
		expected    string
	}{
		{daff.ContentTypeIR, "ImpulseResponse"},
		{daff.ContentTypeMS, "MagnitudeSpectrum"},
		{daff.ContentTypePS, "PhaseSpectrum"},
		{daff.ContentTypeMPS, "MagnitudePhaseSpectrum"},
		{daff.ContentTypeDFT, "DFT"},
	}

	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			if got := tt.contentType.String(); got != tt.expected {
				t.Errorf("ContentType.String() = %v, want %v", got, tt.expected)
			}
		})
	}
}

func TestQuantizationString(t *testing.T) {
	tests := []struct {
		quantization daff.Quantization
		expected     string
	}{
		{daff.QuantizationInt16, "Int16"},
		{daff.QuantizationInt24, "Int24"},
		{daff.QuantizationFloat32, "Float32"},
	}

	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			if got := tt.quantization.String(); got != tt.expected {
				t.Errorf("Quantization.String() = %v, want %v", got, tt.expected)
			}
		})
	}
}

// Note: Integration tests require actual DAFF files.
// Add your test files to the testdata directory and uncomment the following tests:

/*
func TestOpenFile(t *testing.T) {
	reader, err := daff.NewReader()
	if err != nil {
		t.Fatalf("Failed to create reader: %v", err)
	}
	defer reader.Close()

	if err := reader.OpenFile("testdata/example.daff"); err != nil {
		t.Fatalf("Failed to open file: %v", err)
	}

	if !reader.IsValid() {
		t.Fatal("Reader should be valid after opening file")
	}

	reader.CloseFile()

	if reader.IsValid() {
		t.Fatal("Reader should not be valid after closing file")
	}
}

func TestImpulseResponse(t *testing.T) {
	reader, err := daff.NewReader()
	if err != nil {
		t.Fatalf("Failed to create reader: %v", err)
	}
	defer reader.Close()

	if err := reader.OpenFile("testdata/ir_example.daff"); err != nil {
		t.Fatalf("Failed to open file: %v", err)
	}
	defer reader.CloseFile()

	if reader.GetContentType() != daff.ContentTypeIR {
		t.Fatalf("Expected IR content type, got %v", reader.GetContentType())
	}

	ir, err := reader.GetContentIR()
	if err != nil {
		t.Fatalf("Failed to get IR content: %v", err)
	}

	filterLength := ir.GetFilterLength()
	if filterLength <= 0 {
		t.Errorf("Invalid filter length: %d", filterLength)
	}

	samplerate := ir.GetSamplerate()
	if samplerate <= 0 {
		t.Errorf("Invalid samplerate: %d", samplerate)
	}

	recordIndex := ir.GetNearestNeighbour(0, 0)
	coeffs, err := ir.GetFilterCoeffs(recordIndex, 0)
	if err != nil {
		t.Fatalf("Failed to get filter coefficients: %v", err)
	}

	if len(coeffs) != filterLength {
		t.Errorf("Expected %d coefficients, got %d", filterLength, len(coeffs))
	}
}
*/
