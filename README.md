# ✅ Protocheck

Protocheck is a crate that allows you to leverage [`protovalidate`](https://github.com/bufbuild/protovalidate) annotations to automatically generate validation logic for the structs generated from your protobuf messages. 

This allows you to define your validation schemas only once, directly in your protobuf files (or in rust code, using [protoschema](https://github.com/Rick-Phoenix/protoschema)), and then use libraries such as this one or others like [`protovalidate-es`](github.com/bufbuild/protovalidate-es) in the Typescript ecosystem to execute the validation logic.

## ➡️ Getting started 

Visit the [`protocheck-build`](https://crates.io/crates/protocheck-build) docs to learn how to set up protocheck in your `build.rs` script.

## 📓 Noteworthy features

#### 1. It does not require reflection, except at build time. 

This is a major benefit for two reasons. 

First, it removes the need to include a reflection library in the consuming app's binary.

And, most importantly, it avoids the overhead that is introduced by using reflection to determine the structure of a message when validating it.

Rather than using reflection, this crate leverages the [`TryIntoCelValue`](https://docs.rs/protocheck-proc-macro/0.1.0/protocheck_proc_macro/derive.TryIntoCelValue.html) derive macro to generate a method called `try_into_cel_value` which will directly convert any given struct (or field) into the appropriate cel [`Value`](protocheck_core::cel::Value) (only failing in case of a [`Duration`](https://docs.rs/proto-types/0.1.0/proto_types/struct.Duration.html) or [`Timestamp`](https://docs.rs/proto-types/0.1.0/proto_types/struct.Timestamp.html) field being out of the allowed range for [`chrono`](https://docs.rs/chrono/latest/chrono/index.html) types).

#### 2. It uses native rust code for validation except for custom Cel rules. 

Unlike other similar libraries, all of the standard validators are implemented in rust code. This means that the cel interpreter (provided by the [`cel`](https://docs.rs/cel/latest/cel/) crate) is used only for custom rules explicitely defined in Cel, and can be disabled altogether if custom rules are not used. 

#### 3. Extra safety checks for rules definitions

Because of human error, some of these situations may arise:

- A list of allowed values and a list of forbidden values have some items in common
- A string field is trying to use [`BytesRules`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.BytesRules.html)
- An "enum.in" rule (list of allowed values for an enum field) includes values that are not part of that enum
- An "lt" rule (meaning 'less than') specifies a value that is smaller than the "gt" (greater than) rule
- ... and other corner cases

This crate handles these situations by emitting a compilation error, which will report the specific field (and the specific values) involved in the error.

Example:

```proto
message Oopsie {
  string mystring = 1 [(buf.validate.field).string = {
    min_len: 10
    max_len: 2
  }];
}
```

Error message:

`error: Error for field myapp.v1.Oopsie.mystring: min_len cannot be larger than max_len`

#### 4. Strenghtened compile-time safety for Cel programs

When the [`protobuf_validate`](https://docs.rs/protocheck-proc-macro/0.1.0/protocheck_proc_macro/attr.protobuf_validate.html) proc macro is being processed, it will attempt to create a test case for any given Cel expression being used, generating some default values for the given message or field and trying to execute a Cel program with those defaults. 
This ensures that if a Cel expression is fundamentally invalid (for example for a type mismatch), the error will be caught at compile time and not at runtime. (With some caveats explained below)

#### 5. Lazy initialization

All Cel programs are generated using [`LazyLock`](::std::sync::LazyLock) so they are only initialized once. The same thing goes for other static elements being used in the validators, such as regexes or allowed/forbidden list of values.

## ☑️ How to validate messages

After the [`validate`](https://docs.rs/protocheck/0.1.0/protocheck/trait.ProtoValidator.html#tymethod.validate) method has been added to a struct, validating it is as simple as calling `my_struct.validate()`. 

The validate method returns a `Result<(), Violations>`, where the [`Violations`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.Violations.html) struct contains a vector of individual [`Violation`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.Violation.html) elements, which contain the context behind a given validation error, such as the parent messages (if the field was part of a nested message) of the invalid field, along with the error message and the rule id for that given rule.  

Both [`Violations`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.Violations.html) and the invidivual [`Violation`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.Violation.html) structs come with several utility methods, such as [`violation_by_rule_id`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.Violations.html#method.violation_by_rule_id), which allows you to select a particular violation from the list, or [`field_path_str`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.Violation.html#method.field_path_str), which conveniently takes a list of [`FieldPathElement`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/struct.FieldPathElement.html) and turns it into a single string path such as `person.friends.0.address.street_name`.

The [`protocheck-proc-macro`](https://docs.rs/protocheck-proc-macro/0.1.0/protocheck_proc_macro/index.html) crate also adds a generic trait [`ProtoValidator`](https://docs.rs/protocheck/0.1.1/protocheck/trait.ProtoValidator.html) that calls the [`validate`](https://docs.rs/protocheck/0.1.0/protocheck/trait.ProtoValidator.html#tymethod.validate) method.

Example:

>(The examples are taken from the [testing crate](https://github.com/Rick-Phoenix/protocheck/tree/main/tests/src), they show as untested just because it's a separate crate that has its own build script)

```proto
message JediFight {
  Anakin anakin = 1;
  ObiWan obi_wan = 2;
}

message ObiWan {
  bool has_high_ground = 1 [(buf.validate.field).bool.const = true];
}

message Anakin {
  bool has_high_ground = 1 [(buf.validate.field).bool.const = false];
}
```

```rust,ignore
#[test]
fn example() {
  let obi_wan = ObiWan {
    has_high_ground: false,
  };
  let anakin = Anakin {
    has_high_ground: true,
  };
  let jedi_fight = JediFight {
    anakin: Some(anakin),
    obi_wan: Some(obi_wan),
  };

  let Violations { violations } = jedi_fight.validate().unwrap_err();

  violations.iter().for_each(|v| {
    println!(
      "Field path: {}, Error message: {}",
      v.field_path_str().unwrap(),
      v.message()
    )
  });
}
```

Output: 

```text
Field path: anakin.has_high_ground, Error message: must be equal to false
Field path: obi_wan.has_high_ground, Error message: must be equal to true
```

The [tests](https://github.com/Rick-Phoenix/protocheck/tree/main/tests/src) crate contains many other examples of validation schemas being implemented.

## ⚙️ Custom validation with Cel

With the `cel` feature (enabled by default), you can also specify some validation rules defined with the [Cel](https://cel.dev/) syntax, which can be applied to entire structs or to singular fields.

Example:

Let's change the above to this:

```proto
message ObiWan {
  // Message-level validation
  option (buf.validate.message).cel = {
    id: "obi-wan.high_ground"
    message: "obi-wan must have the high ground."
    expression: "this.has_high_ground == true"
  };

  // Field-level validation
  bool has_high_ground = 1 [(buf.validate.field).cel = {
    id: "obi-wan.high_ground"
    message: "obi-wan must have the high ground."
    expression: "this == true"
  }];
}
```

Now the output will be:

```text
Field path: anakin.has_high_ground, Error message: must be equal to false
Field path: obi_wan, Error message: obi-wan must have the high ground.
Field path: obi_wan.has_high_ground, Error message: obi-wan must have the high ground.
```

This is particularly useful for cases when we need to apply message-wide validation which analyzes more than one field at a time. 

The typical example would be:

```proto
message User {
  option (buf.validate.message).cel = {
    id: "passwords_match"
    message: "the two passwords do not match"
    expression: "this.password == this.confirm_password"
  };

  string password = 1;
  string confirm_password = 2;
}
```

```rust,ignore
let user = User {
  password: "abc".to_string(),
  confirm_password: "abcde".to_string(),
};

let Violations { violations } = user.validate().unwrap_err();

println!(
  "Field path: {}, Error message: {}",
  // Message-wide violations do not have a field path unless they are nested in another message  
  // But they do have a rule id
  violations[0].rule_id(),
  violations[0].message()
);
```

Outcome:

```text
Field path: passwords_match, Error message: the two passwords do not match
```

## 📘 Protoschema integration

If you are interested in composing your protobuf files programmatically, and with the benefits of type safety, reusable elements and LSP integration, with a particular focus on making the definition of validation rules a quick and type-safe process, you might want to check out my other crate, [protoschema](https://crates.io/crates/protoschema). 

## 🍸 Usage With Tonic

Check out [this repo](https://github.com/Rick-Phoenix/protocheck-tonic-svelte-example) for a full example of a server that uses `protocheck` to validate incoming grpc requests.

## ⚠️ Caveats and warnings

- The protovalidate rule buf.validate.message.oneof (the one used to make custom oneofs which allow repeated and map fields) is currently not supported.

- While the compile-time check for the validity of a Cel expression helps to catch most if not all errors relative to the Cel program compilation and execution, it is still very encouraged to have some tests that trigger the validation logic at runtime (it's just as easy as calling `.validate()` once again) to be absolutely sure that the Cel program is not causing any issues.

     This is because the Cel validation function can obviously not panic and crash the whole app if a Cel program failed to execute, so it will just return a generic error to the user while logging the actual error. 
     
     This means that if there is an unattended error, then it would silently keep generating these generic and unhelpful error messages for users until it would be reported or noticed in the logs.

     But the good news is that the compile time check prevents the majority of these situations, and adding a very simple test on top of that can eradicate that problem entirely.

- If your message has a reserved rust keyword as a field name, your cel expression should reflect that. So if a field is named `type`, the output struct will have a field named `r#type`, and your cel expression should refer to it using `this['r#type']`, NOT `this.type`.

- For certain cases where the instructions are conflicting but could be intentional, such as using the "const" rule for a field while also having other validators, the other rules will simply be ignored and no error will be shown. This is to allow for cases when you want to have a temporary override for a field's validation without needing to remove the other validators.

- Validation for `bytes` fields only works when using [`bytes::Bytes`](https://docs.rs/bytes/1.10.1/bytes/) as the rust type for them.

- The types for the well known protobuf messages must be imported from [`proto-types`](https://docs.rs/proto-types/0.1.0/proto_types/index.html) (re-exported in this crate in the [`types`] module). These are based on the [`prost-types`](https://docs.rs/prost-types/0.14.1/prost_types/) implementation, with some extra helpers and methods that make validation smoother or even possible at all in some cases. 

     [`compile_protos_with_validators`](https://docs.rs/protocheck-build/0.1.0/protocheck_build/fn.compile_protos_with_validators.html) automatically takes care of calling [`compile_well_known_types`](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.compile_well_known_types) and assigning all of the `.google.protobuf` types to the ones defined in [`proto-types`](https://docs.rs/proto-types/0.1.0/proto_types/index.html). The same thing goes for the types belonging to the [`protovalidate`](https://docs.rs/proto-types/0.1.0/proto_types/protovalidate/index.html) specification.

# 📜 License

Licensed under the MPL-2.0 license.
