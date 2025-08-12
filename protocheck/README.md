# Protocheck

Protocheck is a crate that allows you to leverage [`protovalidate`](https://github.com/bufbuild/protovalidate) annotations to automatically generate validation logic for the structs generated from your protobuf messages. 

# Noteworthy features

1. It does not require reflection, except at build time. 

    This is a major benefit for two reasons. 

    First, it removes the need to include a reflection library in the consuming app's binary.

    And, most importantly, it avoids the overhead that is introduced by using reflection to determine the structure of a message when validating it.

    Rather than using reflection, this crate leverages the [`protocheck-proc-macro::TryIntoCelValue`] derive macro to generate a method called `try_into_cel_value` which will directly convert any given struct (or field) into the appropriate [`cel::Value`](::cel::Value) (only failing in case of a [`Duration`](crate::types::Duration) or [`Timestamp`](crate::types::Timestamp) field being out of range allowed for [chrono](https://docs.rs/chrono/latest/chrono/index.html) types).

2. It uses native rust code for validation except for custom Cel rules. 

    Unlike other similar libraries, all of the standard validators are implemented in rust code. This means that the cel interpreter (provided by the [cel](https://docs.rs/cel/latest/cel/) crate) is used only for custom rules explicitely defined in Cel, and can be disabled altogether if custom rules are not used. 

    Another benefit of not relying purely on Cel and reflection for validation is that the injection of values in a Cel program can be more granular. If only a single field is being validated (with the `(buf.validate.field).cel` directive in protobuf), only that specific field's value will be added to the Cel program, rather than the whole struct, and this can also provide a boost in performance.

3. Type safety (almost) everywhere

    Because of human error, some of these situations may arise:

    - A list of allowed values and a list of forbidden values have some items in common
    - A string field is trying to use [`BytesRules`](crate::types::protovalidate::BytesRules)
    - An "enum.in" rule (list of allowed values for an enum field) includes values that are not part of that enum
    - An "lt" rule (meaning 'less than') specifies a value that is smaller than the "gt" (greater than) rule
    - ... and other corner cases

    This crate handles these situations by emitting a compilation error, which will report the specific field (and the specific values) involved in the error.

4. Compile-time (relative) safety for Cel programs

    When the `protobuf_validate` proc macro is being processed, it will attempt to create a test case for any given Cel expression being used, generating some default values for the given message or field and trying to execute a Cel program with those defaults. This ensures that if a Cel expression is fundamentally invalid (for example for a type mismatch), the error will be caught at compile time and not at runtime. 

5. Lazy initialization

    All Cel programs are generated using `LazyLock` so they are only initialized once. The same thing goes for other static elements being used in the validators, such as regexes or allowed/forbidden list of values (more on that below).

6. Smart list validation

    When a list rule is defined ("in" or "not_in"), the crate will generate two kinds of validators. 

    If the list is up to 15 elements long, it will generate a validator that uses a `slice.contains()` method to check for presence. 
    If the list is longer than that, it will instead use a `HashSet` (also initialized in a `LazyLock`) containing the list of values, to ensure faster lookup times.

7. Precise error paths for oneof fields

    By using the [`protobuf_validate_oneof`](protocheck_proc_macro::protobuf_validate_oneof) proc macro and the [`Oneof`](protocheck_proc_macro::Oneof) derive macro, this crate will add a `proto_name` attribute to the variants of a generated oneof enum, so that error messages can accurately report the specific name of the invalid field, rather than the name of the oneof itself.

# How to validate messages

After the `validate` method has been added to a struct, validating it is as simple as calling `my_struct.validate()`. 

The validate method returns a `Result<(), Violations>`, where the [`Violations`] struct contains a vector of individual [`Violation`](crate::types::protovalidate::Violation) elements, which contain the context behind a given validation error, such as the parent messages (if the field was part of a nested message) of the invalid field, along with the error message and the rule id for that given rule.  

Both [`Violations`] and the invidivual [`Violation`](crate::types::protovalidate::Violation) structs come with several utility methods, such as [`violation_by_rule_id`](crate::types::protovalidate::Violations::violation_by_rule_id), which allows you to select a particular violation from the list, or [`field_path`](crate::types::protovalidate::Violation::field_path), which conveniently turns a list of [`FieldPathElement`](crate::types::protovalidate::FieldPathElement) and turns it into a single string path such as `person.friends.0.address.street_name`.

The [`protocheck-proc-macro`](::protocheck-proc-macro) crate also adds a generic trait [`ProtoValidator`] that calls the `validate` method, which can be used to develop things like a Tower layer that accepts any struct with a validator, calls the `validate` method and returns the result.

The [tests](https://github.com/Rick-Phoenix/protocheck/tree/main/tests/src) crate contains many examples of validation outcomes.

# Custom validation with Cel

With the `cel` feature (enabled by default), you can also specify some validation rules defined with the [Cel](https://cel.dev/) syntax, which can be applied to entire structs or to singular fields.

# Getting started 

To get started, you need to use [`protocheck-build`](::protocheck-build) as a build dependency in your crate, which will use [`protocheck-proc-macro`] to add all the validation logic to your structs. The setup will look more or less like this (this is taken directly from the tests crate)

```rust
use prost_build::Config;
use protocheck_build::compile_protos_with_validators;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not find OUT_DIR"));
  let final_descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  let proto_files = &["proto/myapp/v1/tests.proto"];

  let mut config = Config::new();
  config
    .file_descriptor_set_path(final_descriptor_path.clone())
    // Enable the use of bytes::Bytes for `bytes` fields
    .bytes(["."])
    .enable_type_names()
    // Add any extra custom attributes
    .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
    .out_dir(out_dir.clone());

  // Call the build helper
  compile_protos_with_validators(&mut config, proto_files, proto_include_paths, &["myapp.v1"])?;

  // Compile protos
  config.compile_protos(proto_files, proto_include_paths)?;

  // Set the env for the file descriptor location
  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    final_descriptor_path.display()
  );

  Ok(())
}
```

[`compile_protos_with_validators`] takes these arguments:

1. The [`config`](::prost-build::Config) struct.
2. The proto files and include paths being used by the `config`.
3. The list of packages to apply validators to. If a given message contains Cel validation or is validated as a field by another message, its package name must be included in this list.

The function will then create an intermediary descriptor, iterate its messages, and use the information extracted from them to add the derives and attributes to the actual `config` that are needed by [`protocheck-proc-macro`] to add the validation logic.

# Caveats

- While the compile-time check for the validity of a Cel expression helps to catch most if not all errors relative to the Cel program compilation and execution, it is still very encouraged to have some tests that trigger the validation logic at runtime (it's just as easy as calling `.validate()` once again) to be absolutely sure that the Cel program is not causing any issues.

 This is because the Cel validation function can obviously not panic and crash the whole app if a Cel program failed to execute, so it will just return a generic error to the user while logging the actual error. 
 
 This means that if there is an unattended error, then it would silently keep generating these generic and unhelpful error messages for users until it would be reported or noticed in the logs.

 But the good news is that the compile time check prevents the majority of these situations, and adding a very simple test on top of that can eradicate that problem entirely.

- Validation for `bytes` fields only works when using [`bytes::Bytes`](https://docs.rs/bytes/1.10.1/bytes/) as the rust type for them.

- The types for the well known protobuf messages must be imported from [`proto-types`]. These are based on the [`prost-types`](https://docs.rs/prost-types/0.14.1/prost_types/) implementation, with some extra helpers and methods that make validation smoother or even possible at all in some cases. 

[`compile_protos_with_validators`] automatically takes care of calling `compile_well_known_types` and assign all of the `.google.protobuf` types to the ones described in [`proto-types`] (and re-exported in this crate under the [`types`] module). The same thing goes for the types coming from the `protovalidate` specification.


