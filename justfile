test:
    cargo test --all-features -p tests -p protocheck-core -p proto-types  -- -q --nocapture

release version exec="": test
    ./pre_release.sh {{ version }} {{ exec }}
    cargo release {{ version }} --exclude proto-types {{ exec }}

[working-directory('proto_types')]
release-proto-types version exec="": test
    ../pre_release.sh {{ version }} {{ exec }}
    cargo release {{ version }} {{ exec }}
