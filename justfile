test:
    cargo test --all-features -- -q --nocapture

release version exec="": test
    ./pre_release.sh {{ version }} {{ exec }}

[working-directory('proto_types')]
release-proto-types version exec="": test
    ../pre_release.sh {{ version }} {{ exec }}
