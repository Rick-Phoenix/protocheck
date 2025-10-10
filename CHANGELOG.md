## [0.1.11] - 2025-10-10

### 📚 Documentation

- Changed doc_auto_cfg to doc_cfg
- Using the readme link for protocheck-build instead of the docs.rs link
## [protocheck-v0.1.10] - 2025-10-09

### 🐛 Bug Fixes

- Using escaped rust keywords for field names in cel compilation check
- Added optional serde for bytes fields

### 🚜 Refactor

- Using stderr for logging runtime errors

### 📚 Documentation

- Specifying that the proc-macro crate is not needed as a dependency
- Added disclaimer about unsupported rule
- Moved build documentation to protocheck-build to run tests with the example
- Added further checks for examples that are part of the testing crate
- Added example for tonic

### 🧪 Testing

- Add all testing outputs to the repo

### 📦 CI/CD

- Prepping a more robust workflow for release
- Added changelog
- Updated release workflow
