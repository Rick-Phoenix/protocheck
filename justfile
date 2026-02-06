test:
    cargo test --all-features -p tests -p protocheck-core -- -q --nocapture

release version exec="": test
    ./pre_release.sh {{ version }} {{ exec }}
    cargo release {{ version }} {{ exec }}
