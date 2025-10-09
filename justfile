test:
    cargo test --all-features -- -q --nocapture

release-test version:
    ./pre_release.sh {{ version }}

[confirm]
release-exec version:
    ./pre_release.sh {{ version }} --exec
