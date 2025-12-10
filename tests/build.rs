use std::{env, path::PathBuf};

use prost_build::Config;
use protocheck_build::{compile_protos_with_validators, get_proto_files_recursive};
use protoschema::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let package = Package::new("myapp.v1");
  let file = package.new_file("common_types_tests");
  let file2 = package.new_file("tests2");

  message!(
    file2.new_message("RustKeywordsInFields"),
    cel = [
      { id = "abc", msg = "abc", expr = "this['r#type'] == 'abc'" }
    ],

    1 => string!("type", |v| v.cel([ cel_rule!(id = "abc", msg = "abc", expr = "this == 'abc'")])),
    2 => string!("enum", |v| v.cel([ cel_rule!(id = "abc", msg = "abc", expr = "this == 'abc'")])),
    3 => string!("const", |v| v.cel([ cel_rule!(id = "abc", msg = "abc", expr = "this == 'abc'")])),
    4 => string!("crate", |v| v.cel([ cel_rule!(id = "abc", msg = "abc", expr = "this == 'abc'")])),
    5 => string!("pub", |v| v.cel([ cel_rule!(id = "abc", msg = "abc", expr = "this == 'abc'")])),
    6 => string!("struct", |v| v.cel([ cel_rule!(id = "abc", msg = "abc", expr = "this == 'abc'")])),
  );

  message!(
    file.new_message("CommonTypesTests"),

    1 => time_of_day!("time_of_day", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.hours > 12") ])),
    2 => localized_text!("localized_text", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.language_code == 'en-US'") ])),
    3 => quaternion!("quaternion", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.x == 23.0") ])),
    4 => money!("money", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.units == 23") ])),

    5 => date_time!("datetime", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.year == 1000") ])),
    6 => time_zone!("timezone", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.id == 'America/New_York'") ])),
    7 => lat_lng!("lat_lng", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.latitude == 23.0") ])),
    8 => postal_address!("postal_address", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.region_code == 'AUS'") ])),
    9 => date!("date", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.year == 1000") ])),
    10 => interval!("interval", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.start_time == timestamp('2023-01-01T00:00:00Z')") ])),
    11 => expr!("expr", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.title == 'an expr within an expr, spooky!'") ])),
    12 => color!("color", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.red == 1.0") ])),
    13 => fraction!("fraction", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.denominator != 0") ])),
    14 => decimal!("decimal", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.value == '0.10'") ])),
    15 => phone_number!("phone_number", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.extension == '123'") ])),
  );

  message!(
    file.new_message("RpcTypesTests"),

    1 => error_info!("error_info", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.reason == 'dunno'") ])),
    2 => retry_info!("retry_info", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.retry_delay == duration('14h')") ])),
    3 => quota_failure!("quota_failure", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "size(this.violations) == 15") ])),
    4 => quota_failure_violation!("quota_failure_violation", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.subject == 'dunno'") ])),
    5 => precondition_failure!("precondition_failure", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "size(this.violations) == 15") ])),
    6 => precondition_failure_violation!("precondition_failure_violation", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.subject == 'dunno'") ])),
    7 => bad_request!("bad_request", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "size(this.field_violations) == 15") ])),
    8 => field_violation!("field_violation", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.field == 'and I said...'") ])),
    9 => request_info!("request_info", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.request_id == 'heeyeey, heyyeyey, I said hey,'") ])),
    10 => resource_info!("resource_info", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.resource_name == 'whats going on?'") ])),
    11 => help!("help", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "size(this.links) == 15") ])),
    12 => link!("link", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.url == 'rick_roll.com'") ])),
    13 => localized_message!("localized_message", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.locale == 'en-US'") ])),
    14 => http_request!("http_request", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.method == 'GET'") ])),
    15 => http_response!("http_response", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.status == 200") ])),
    16 => http_header!("http_header", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.key == 'Content-Type'") ])),
    17 => status!("status", |v| v.cel([ cel_rule!(id = "id", msg = "msg", expr = "this.code == 200") ])),
  );

  let root = PathBuf::from("proto");
  package.render_templates(&root)?;

  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = env::var("OUT_DIR")
    .map(PathBuf::from)
    .unwrap_or(env::temp_dir());
  let descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  let files = get_proto_files_recursive(PathBuf::from("proto/myapp/v1"))?;

  let mut config = Config::new();
  config
    .file_descriptor_set_path(&descriptor_path)
    .extern_path(".google.type", "::proto_types")
    .extern_path(".google.rpc", "::proto_types")
    .bytes(["."])
    .enable_type_names()
    .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
    .out_dir(&out_dir);

  compile_protos_with_validators(&mut config, &files, proto_include_paths, &["myapp.v1"])?;

  config.compile_protos(&files, proto_include_paths)?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    descriptor_path.display()
  );

  Ok(())
}
