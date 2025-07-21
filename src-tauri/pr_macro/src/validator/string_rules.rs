use super::CelRule;
use super::CelRuleValue;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proto_types::buf::validate;
use proto_types::buf::validate::StringRules;
use proto_types::FieldData;
use quote::quote;
use regex::Regex;

pub fn get_string_rules(
  field_data: FieldData,
  string_rules: &StringRules,
) -> Result<Vec<TokenStream>, Box<dyn std::error::Error>> {
  let mut rules: Vec<TokenStream> = Vec::new();

  let field_name = field_data.name.clone();
  let field_ident = Ident::new(&field_name, Span::call_site());

  if string_rules.max_len.is_some() {
    let max_len_value = string_rules.max_len.unwrap() as usize;

    if field_data.is_repeated {
      let idx = Ident::new("idx", Span::call_site());
      let item = Ident::new("item", Span::call_site());

      let expr = quote! {
        for (#idx, #item) in self.#field_ident.iter().enumerate() {
          match macro_impl::validators::strings::max_len(#field_data, #item, #max_len_value) {
            Ok(_) => {},
            Err(v) => violations.push(v),
          };
        };
      };

      rules.push(expr);
    } else {
      let expr = quote! {
        match macro_impl::validators::strings::max_len(#field_data, &self.#field_ident, #max_len_value) {
          Ok(_) => {},
          Err(v) => violations.push(v),
        };
      };
      rules.push(expr);
    }
  }

  Ok(rules)
}

// pub fn get_string_rules(
//   string_rules: &StringRules,
// ) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
//   let mut rules: Vec<CelRule> = Vec::new();
//
//   if string_rules.r#const.is_some() {
//     let const_value = string_rules.r#const.clone().unwrap();
//
//     let (expression, message) = super::COMMON_RULES.get("const").unwrap();
//     rules.push(CelRule {
//       id: "string.const".to_string(),
//       message: message.to_string(),
//       expression: expression.to_string(),
//       value: CelRuleValue::String(const_value),
//     });
//   }
//
//   if string_rules.len.is_some() {
//     let len_value = string_rules.len.unwrap();
//
//     rules.push(CelRule {
//       id: "string.len".to_string(),
//       message: "".to_string(),
//       expression: "uint(this.size()) != rules.len ? 'value length must be %s characters'.format([rules.len]) : ''".to_string(),
//       value: CelRuleValue::U64(len_value),
//     });
//   }
//
//   if string_rules.min_len.is_some() {
//     let min_len_value = string_rules.min_len.unwrap();
//
//     rules.push(CelRule {
//       id: "string.min_len".to_string(),
//       message: "".to_string(),
//       expression: "uint(this.size()) < rules.min_len ? 'value length must be at least %s characters'.format([rules.min_len]) : ''".to_string(),
//       value: CelRuleValue::U64(min_len_value),
//     });
//   }
//
//   if string_rules.max_len.is_some() {
//     let max_len_value = string_rules.max_len.unwrap();
//
//     rules.push(CelRule {
//       id: "string.max_len".to_string(),
//       message: "".to_string(),
//       expression: "uint(this.size()) > rules.max_len ? 'value length must be at most %s characters'.format([rules.max_len]) : ''".to_string(),
//       value: CelRuleValue::U64(max_len_value),
//     });
//   }
//
//   if string_rules.len_bytes.is_some() {
//     let len_bytes_value = string_rules.len_bytes.unwrap();
//
//     rules.push(CelRule {
//       id: "string.len_bytes".to_string(),
//       message: "".to_string(),
//       expression: "uint(bytes(this).size()) != rules.len_bytes ? 'value length must be %s bytes'.format([rules.len_bytes]) : ''".to_string(),
//       value: CelRuleValue::U64(len_bytes_value),
//     });
//   }
//
//   if string_rules.min_bytes.is_some() {
//     let min_bytes_value = string_rules.min_bytes.unwrap();
//
//     rules.push(CelRule {
//       id: "string.min_bytes".to_string(),
//       message: "".to_string(),
//       expression: "uint(bytes(this).size()) < rules.min_bytes ? 'value length must be at least %s bytes'.format([rules.min_bytes]) : ''".to_string(),
//       value: CelRuleValue::U64(min_bytes_value),
//     });
//   }
//
//   if string_rules.max_bytes.is_some() {
//     let max_bytes_value = string_rules.max_bytes.unwrap();
//
//     rules.push(CelRule {
//       id: "string.max_bytes".to_string(),
//       message: "".to_string(),
//       expression: "uint(bytes(this).size()) > rules.max_bytes ? 'value length must be at most %s bytes'.format([rules.max_bytes]) : ''".to_string(),
//       value: CelRuleValue::U64(max_bytes_value),
//     });
//   }
//
//   if string_rules.pattern.is_some() {
//     let pattern_value = string_rules.pattern.clone().unwrap();
//
//     let compiled_regex = Regex::new(&pattern_value)?;
//     rules.push(CelRule {
//       id: "string.pattern".to_string(),
//       message: "".to_string(),
//       expression: "!this.matches(rules.pattern) ? 'value does not match regex pattern `%s`'.format([rules.pattern]) : ''".to_string(),
//       value: CelRuleValue::Regex(Box::new(compiled_regex)),
//     });
//   }
//
//   if string_rules.prefix.is_some() {
//     let prefix_value = string_rules.prefix.clone().unwrap();
//
//     rules.push(CelRule {
//       id: "string.prefix".to_string(),
//       message: "".to_string(),
//       expression: "!this.startsWith(rules.prefix) ? 'value does not have prefix `%s`'.format([rules.prefix]) : ''".to_string(),
//       value: CelRuleValue::String(prefix_value),
//     });
//   }
//
//   if string_rules.suffix.is_some() {
//     let suffix_value = string_rules.suffix.clone().unwrap();
//
//     rules.push(CelRule {
//       id: "string.suffix".to_string(),
//       message: "".to_string(),
//       expression: "!this.endsWith(rules.suffix) ? 'value does not have suffix `%s`'.format([rules.suffix]) : ''".to_string(),
//       value: CelRuleValue::String(suffix_value),
//     });
//   }
//
//   if string_rules.contains.is_some() {
//     let contains_value = string_rules.contains.clone().unwrap();
//
//     rules.push(CelRule {
//       id: "string.contains".to_string(),
//       message: "".to_string(),
//       expression: "!this.contains(rules.contains) ? 'value does not contain substring `%s`'.format([rules.contains]) : ''".to_string(),
//       value: CelRuleValue::String(contains_value),
//     });
//   }
//
//   if string_rules.not_contains.is_some() {
//     let not_contains_value = string_rules.not_contains.clone().unwrap();
//
//     rules.push(CelRule {
//       id: "string.not_contains".to_string(),
//       message: "".to_string(),
//       expression: "!this.contains(rules.contains) ? 'value does not contain substring `%s`'.format([rules.contains]) : ''".to_string(),
//       value: CelRuleValue::String(not_contains_value),
//     });
//   }
//
//   if string_rules.r#in.len() > 0 {
//     let in_value = string_rules.r#in.clone();
//
//     let (expression, message) = super::COMMON_RULES.get("in").unwrap();
//     rules.push(CelRule {
//       id: "string.in".to_string(),
//       message: message.to_string(),
//       expression: expression.to_string(),
//       value: CelRuleValue::RepeatedString(in_value),
//     });
//   }
//
//   if string_rules.not_in.len() > 0 {
//     let not_in_value = string_rules.not_in.clone();
//
//     let (expression, message) = super::COMMON_RULES.get("not_in").unwrap();
//     rules.push(CelRule {
//       id: "string.not_in".to_string(),
//       message: message.to_string(),
//       expression: expression.to_string(),
//       value: CelRuleValue::RepeatedString(not_in_value),
//     });
//   }
//
//   if let Some(well_known) = string_rules.well_known {
//     let mut get_well_known =
//       |wk: &str, message: &str, expression: &str| -> Result<(), Box<dyn std::error::Error>> {
//         rules.push(CelRule {
//           id: format!("string.{}", wk).to_string(),
//           message: message.to_string(),
//           expression: expression.to_string(),
//           value: CelRuleValue::Bool(true),
//         });
//
//         Ok(())
//       };
//     match well_known {
//       validate::string_rules::WellKnown::Email(_) => {
//         get_well_known(
//           "email",
//           "value must be a valid email address",
//           "!rules.email || this == '' || this.isEmail()",
//         )?;
//       }
//       validate::string_rules::WellKnown::Hostname(_) => {
//         get_well_known(
//           "hostname",
//           "value must be a valid hostname",
//           "!rules.hostname || this == '' || this.isHostname()",
//         )?;
//       }
//       validate::string_rules::WellKnown::Ip(_) => {
//         get_well_known(
//           "ip",
//           "value must be a valid IP address",
//           "!rules.ip || this == '' || this.isIp()",
//         )?;
//       }
//       validate::string_rules::WellKnown::Ipv4(_) => {
//         get_well_known(
//           "ipv4",
//           "value must be a valid IPv4 address",
//           "!rules.ipv4 || this == '' || this.isIp(4)",
//         )?;
//       }
//       validate::string_rules::WellKnown::Ipv6(_) => {
//         get_well_known(
//           "ipv6",
//           "value must be a valid IPv6 address",
//           "!rules.ipv6 || this == '' || this.isIp(6)",
//         )?;
//       }
//       validate::string_rules::WellKnown::Uri(_) => {
//         get_well_known(
//           "uri",
//           "value must be a valid URI",
//           "!rules.uri || this == '' || this.isUri()",
//         )?;
//       }
//       validate::string_rules::WellKnown::UriRef(_) => {
//         get_well_known(
//           "uri_ref",
//           "value must be a valid URI Reference",
//           "!rules.uri_ref || this.isUriRef()",
//         )?;
//       }
//       validate::string_rules::WellKnown::Address(_) => {
//         get_well_known(
//           "address",
//           "value must be a valid hostname, or ip address",
//           "!rules.address || this == '' || this.isHostname() || this.isIp()",
//         )?;
//       }
//       validate::string_rules::WellKnown::Uuid(_) => {
//         get_well_known("uuid", "value must be a valid UUID","!rules.uuid || this == '' || this.matches('^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$')")?;
//       }
//       validate::string_rules::WellKnown::Tuuid(_) => {
//         get_well_known(
//           "tuuid",
//           "value must be a valid trimmed UUID",
//           "!rules.tuuid || this == '' || this.matches('^[0-9a-fA-F]{32}$')",
//         )?;
//       }
//       validate::string_rules::WellKnown::IpWithPrefixlen(_) => {
//         get_well_known(
//           "ip_with_prefixlen",
//           "value must be a valid IP prefix",
//           "!rules.ip_with_prefixlen || this == '' || this.isIpPrefix()",
//         )?;
//       }
//       validate::string_rules::WellKnown::Ipv4WithPrefixlen(_) => {
//         get_well_known(
//           "ipv4_with_prefixlen",
//           "value must be a valid IPv4 address with prefix length",
//           "!rules.ipv4_with_prefixlen || this == '' || this.isIpPrefix(4)",
//         )?;
//       }
//       validate::string_rules::WellKnown::Ipv6WithPrefixlen(_) => {
//         get_well_known(
//           "ipv6_with_prefixlen",
//           "value must be a valid IPv6 address with prefix length",
//           "!rules.ipv6_with_prefixlen || this == '' || this.isIpPrefix(6)",
//         )?;
//       }
//       validate::string_rules::WellKnown::IpPrefix(_) => {
//         get_well_known(
//           "ip_prefix",
//           "value must be a valid IP prefix",
//           "!rules.ip_prefix || this == '' || this.isIpPrefix(true)",
//         )?;
//       }
//       validate::string_rules::WellKnown::Ipv4Prefix(_) => {
//         get_well_known(
//           "ipv4_prefix",
//           "value must be a valid IPv4 prefix",
//           "!rules.ipv4_prefix || this == '' || this.isIpPrefix(4, true)",
//         )?;
//       }
//       validate::string_rules::WellKnown::Ipv6Prefix(_) => {
//         get_well_known(
//           "ipv6_prefix",
//           "value must be a valid IPv6 prefix",
//           "!rules.ipv6_prefix || this == '' || this.isIpPrefix(6, true)",
//         )?;
//       }
//       validate::string_rules::WellKnown::HostAndPort(_) => {
//         get_well_known(
//           "host_and_port",
//           "value must be a valid host (hostname or IP address) and port pair",
//           "!rules.host_and_port || this == '' || this.isHostAndPort(true)",
//         )?;
//       }
//       validate::string_rules::WellKnown::WellKnownRegex(well_known_regex_id) => {
//         match well_known_regex_id {
//           1 => {
//             get_well_known(
//               "well_known_regex.header_name",
//               "value must be a valid HTTP header name",
//               r"rules.well_known_regex != 1 || this == '' || this.matches(!has(rules.strict) || rules.strict ? '^:?[0-9a-zA-Z!#$%&\\'*+-.^_|~\\x60]+$' : '^[^\\u0000\\u000A\\u000D]+$')",
//             )?;
//           }
//           2 => {
//             get_well_known(
//               "well_known_regex.header_value",
//               "value must be a valid HTTP header value",
//               r"rules.well_known_regex != 2 || this.matches(!has(rules.strict) || rules.strict ? '^[^\\u0000-\\u0008\\u000A-\\u001F\\u007F]*$' : '^[^\\u0000\\u000A\\u000D]*$')",
//             )?;
//           }
//           _ => {}
//         }
//       }
//     };
//   }
//
//   Ok(rules)
// }
