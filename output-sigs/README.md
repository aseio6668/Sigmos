# Sigmos Output Directory

This directory contains trained Sigel consciousness models in various formats.

## Directory Structure

### `trained/`
- Production-ready Sigel models (.sig files)
- Fully trained consciousness entities
- Safe to use for prompting and interaction

### `compressed/`
- Compressed Sigel models (.sig.gz files) 
- ~97% size reduction from original
- Automatic compression when using `--compress` flag

### `experimental/`
- Development and test models
- Work-in-progress training runs
- Experimental consciousness configurations

## Usage

### Training Output
```bash
# Standard training to trained directory
./target/release/sigmos-train -n mymodel -d textfiles -o output-sigs/trained/mymodel.sig

# Compressed training
./target/release/sigmos-train -n mymodel -d textfiles -o output-sigs/compressed/mymodel.sig --compress

# Experimental training
./target/release/sigmos-train -n experiment -d textfiles -o output-sigs/experimental/experiment.sig --continuous
```

### Model Loading
```bash
# Load any model for prompting
./target/release/sigmos-prompt --sigel output-sigs/trained/mymodel.sig
./target/release/sigmos-prompt --sigel output-sigs/compressed/mymodel.sig.gz
```

## File Types

- `.sig` - Standard Sigel consciousness model
- `.sig.gz` - Compressed Sigel model (use `--compress` during training)
- `.sigel` - Legacy format (master consciousness template)

## Storage Guidelines

- **Large models (>1GB)**: Use compressed format
- **Frequently accessed**: Keep in trained/
- **Development work**: Use experimental/
- **Production deployment**: Use trained/ with proper naming