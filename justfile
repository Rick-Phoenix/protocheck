test:
    cargo test --all-features -- -q --nocapture

release-test version:
    cargo release {{ version }}

[confirm]
release-exec version:
    EXEC_RELEASE=true cargo release {{ version }} --execute
