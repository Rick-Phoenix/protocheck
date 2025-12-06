use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct ValidationData<'a> {
  pub full_name: &'a str,
  pub proto_name: &'a str,
  pub ignore: Ignore,
  pub tag: u32,
  pub is_required: bool,
  pub is_optional: bool,
  pub is_in_oneof: bool,
  pub is_boxed: bool,
  pub field_span: Span,

  // These two are always needed to populate
  // the protovalidate data, for all types
  pub map_keys_type: Option<ProtoType>,
  pub map_values_type: Option<ProtoType>,

  pub map_key_ident: &'a Ident,
  pub map_value_ident: &'a Ident,
  pub index_ident: &'a Ident,
  pub item_ident: &'a Ident,
  pub item_rust_ident: &'a Ident,
  pub violations_ident: &'a Ident,
  pub parent_messages_ident: &'a Ident,
  pub field_context_ident: &'a Ident,
  pub map_key_context_ident: &'a Ident,
  pub map_value_context_ident: &'a Ident,
  pub vec_item_context_ident: &'a Ident,
  pub field_kind: FieldKind,
  pub value_ident: OnceCell<TokenStream2>,
}

pub struct RepeatedValidator {
  pub vec_level_rules: TokenStream2,
  pub items_rules: TokenStream2,
}

pub struct MapValidator {
  pub map_level_rules: TokenStream2,
  pub keys_rules: TokenStream2,
  pub values_rules: TokenStream2,
}

pub enum ListRule {
  In,
  NotIn,
}

impl ListRule {
  pub fn name(&self) -> String {
    match self {
      Self::In => "in".to_string(),
      Self::NotIn => "not_in".to_string(),
    }
  }
}

impl ValidationData<'_> {
  pub fn static_full_name(&self) -> String {
    self
      .full_name
      .to_case(Case::Snake)
      .replace(".", "_")
      .to_uppercase()
  }

  pub fn get_substring_validator(&self, tokens: &mut TokenStream2, rules: SubstringRules) {
    let value_ident = self.value_ident();
    let field_context_ident = self.field_context_ident();
    let validator_type_ident = format_ident!("{}", self.field_kind.inner_type().name());

    if let Some(contains) = rules.contains {
      let SubstringRule {
        error_message,
        val_tokens,
      } = contains;
      let expr = quote! {
        ::protocheck::validators::#validator_type_ident::contains(&#field_context_ident, &#value_ident, #val_tokens, #error_message)
      };

      self.get_validator_tokens(tokens, &expr);
    }

    if let Some(not_contains) = rules.not_contains {
      let SubstringRule {
        error_message,
        val_tokens,
      } = not_contains;
      let expr = quote! {
        ::protocheck::validators::#validator_type_ident::not_contains(&#field_context_ident, &#value_ident, #val_tokens, #error_message)
      };

      self.get_validator_tokens(tokens, &expr);
    }

    if let Some(prefix) = rules.prefix {
      let SubstringRule {
        error_message,
        val_tokens,
      } = prefix;
      let expr = quote! {
        ::protocheck::validators::#validator_type_ident::prefix(&#field_context_ident, &#value_ident, #val_tokens, #error_message)
      };

      self.get_validator_tokens(tokens, &expr);
    }

    if let Some(suffix) = rules.suffix {
      let SubstringRule {
        error_message,
        val_tokens,
      } = suffix;
      let expr = quote! {
        ::protocheck::validators::#validator_type_ident::suffix(&#field_context_ident, &#value_ident, #val_tokens, #error_message)
      };

      self.get_validator_tokens(tokens, &expr);
    }
  }

  pub fn get_length_validator(&self, tokens: &mut TokenStream2, rules: LengthRules) {
    let value_ident = self.value_ident();
    let field_context_ident = self.field_context_ident();
    let validator_type_ident = format_ident!("{}", rules.name());
    let unit = rules.unit();

    if let Some(len) = rules.len {
      let error_message = format!(
        "must be exactly {} {}{} long",
        len,
        unit,
        get_plural_suffix(len)
      );
      let func_name = format_ident!("{}", rules.len_name());

      let expr = quote! {
        ::protocheck::validators::#validator_type_ident::#func_name(&#field_context_ident, &#value_ident, #len, #error_message)
      };

      self.get_validator_tokens(tokens, &expr);
    }

    if let Some(min_len) = rules.min_len {
      let error_message = format!(
        "must contain at least {} {}{}",
        min_len,
        unit,
        get_plural_suffix(min_len)
      );
      let func_name = format_ident!("{}", rules.min_len_name());

      let expr = quote! {
        ::protocheck::validators::#validator_type_ident::#func_name(&#field_context_ident, &#value_ident, #min_len, #error_message)
      };

      self.get_validator_tokens(tokens, &expr);
    }

    if let Some(max_len) = rules.max_len {
      let error_message = format!(
        "cannot contain more than {} {}{}",
        max_len,
        unit,
        get_plural_suffix(max_len)
      );
      let func_name = format_ident!("{}", rules.max_len_name());

      let expr = quote! {
        ::protocheck::validators::#validator_type_ident::#func_name(&#field_context_ident, &#value_ident, #max_len, #error_message)
      };

      self.get_validator_tokens(tokens, &expr);
    }
  }

  pub fn get_regex_validator(
    &self,
    tokens: &mut TokenStream2,
    static_defs: &mut TokenStream2,
    regex: &str,
  ) {
    let value_ident = self.value_ident();
    let field_context_ident = self.field_context_ident();

    let static_regex_ident = format_ident!("__{}_REGEX", self.static_full_name());
    static_defs.extend(quote! {
      static #static_regex_ident: ::std::sync::LazyLock<::regex::Regex> = ::std::sync::LazyLock::new(|| {
        ::regex::Regex::new(#regex).unwrap()
      });
    });

    let validator_type_ident = format_ident!("{}", self.field_kind.inner_type().name());
    let error_message = format!("must match the following regex: `{}`", regex);

    let validator_expression_tokens = quote! {
      ::protocheck::validators::#validator_type_ident::pattern(&#field_context_ident, &#value_ident, &#static_regex_ident, #error_message)
    };
    self.get_validator_tokens(tokens, &validator_expression_tokens);
  }

  pub fn get_comparable_validator<T>(
    &self,
    tokens: &mut TokenStream2,
    comparable_rules: &ComparableRules<T>,
  ) where
    T: ToTokens + Display + PartialEq + PartialOrd + Debug,
  {
    let module_path = quote! { ::protocheck::validators::comparables };
    let field_context_ident = self.field_context_ident();
    let proto_type_name = self.field_kind.inner_type().name();
    let value_ident = self.value_ident();

    if let Some(less_than) = comparable_rules.less_than.as_ref() {
      match less_than {
        ComparableLessThan::Lt {
          val: lt,
          error_message,
        } => {
          let func_ident = format_ident!("{proto_type_name}_lt");

          let expr = quote! { #module_path::#func_ident(&#field_context_ident, #value_ident, #lt, #error_message) };
          self.get_validator_tokens(tokens, &expr);
        }

        ComparableLessThan::Lte {
          val: lte,
          error_message,
        } => {
          let func_ident = format_ident!("{proto_type_name}_lte");

          let expr = quote! { #module_path::#func_ident(&#field_context_ident, #value_ident, #lte, #error_message) };
          self.get_validator_tokens(tokens, &expr);
        }
      };
    }

    if let Some(greater_than) = comparable_rules.greater_than.as_ref() {
      match greater_than {
        ComparableGreaterThan::Gt {
          val: gt,
          error_message,
        } => {
          let func_ident = format_ident!("{proto_type_name}_gt");

          let expr = quote! { #module_path::#func_ident(&#field_context_ident, #value_ident, #gt, #error_message) };
          self.get_validator_tokens(tokens, &expr);
        }
        ComparableGreaterThan::Gte {
          val: gte,
          error_message,
        } => {
          let func_ident = format_ident!("{proto_type_name}_gte");

          let expr = quote! { #module_path::#func_ident(&#field_context_ident, #value_ident, #gte, #error_message) };
          self.get_validator_tokens(tokens, &expr);
        }
      };
    }
  }

  pub fn get_list_validator(
    &self,
    rule_type: ListRule,
    tokens: &mut TokenStream2,
    list: ItemList,
    static_defs: &mut TokenStream2,
  ) {
    let field_context_ident = self.field_context_ident();
    let value_ident = self.value_ident();
    let proto_type_name = self.field_kind.inner_type().name();

    let module_path = quote! { ::protocheck::validators::containing };
    let rule_name = rule_type.name();

    match list {
      ItemList::Slice {
        error_message,
        tokens: list_tokens,
      } => {
        let func_path = format_ident!("{proto_type_name}_{rule_name}_slice_list");
        let expr = quote! {
          #module_path::#func_path(&#field_context_ident, #value_ident, &#list_tokens, #error_message)
        };

        self.get_validator_tokens(tokens, &expr);
      }
      ItemList::HashSet {
        error_message,
        tokens: hashset_tokens,
        static_ident,
      } => {
        let func_path = format_ident!("{proto_type_name}_{rule_name}_hashset_list");

        static_defs.extend(hashset_tokens);

        let expr = quote! {
          #module_path::#func_path(&#field_context_ident, #value_ident, &#static_ident, #error_message)
        };

        self.get_validator_tokens(tokens, &expr);
      }
    };
  }

  pub fn field_context_tokens(
    &self,
    field_kind: FieldKind,
    field_context_ident: &Ident,
  ) -> TokenStream2 {
    let Self {
      parent_messages_ident,
      proto_name,
      tag,
      ..
    } = self;

    let key_type_tokens = self.map_keys_type.map_or_else(
      || quote! { None },
      |key_type| {
        quote! { Some(#key_type) }
      },
    );

    let value_type_tokens = self.map_values_type.map_or_else(
      || quote! { None },
      |value_type| {
        quote! { Some(#value_type) }
      },
    );

    let subscript_tokens = self.subscript_tokens(field_kind);

    quote! {
      let #field_context_ident = ::protocheck::field_data::FieldContext {
        parent_elements: #parent_messages_ident.as_slice(),
        subscript: #subscript_tokens,
        key_type: #key_type_tokens,
        value_type: #value_type_tokens,
        field_kind: #field_kind,
        proto_name: #proto_name,
        tag: #tag,
      };
    }
  }

  pub fn aggregate_map_rules(&self, tokens: &mut TokenStream2, rules_data: &MapValidator) {
    let MapValidator {
      map_level_rules,
      keys_rules,
      values_rules,
    } = rules_data;

    // At this point, the rules have already been gathered, all we do here
    // is simply injecting the context for the keys/values/map so that
    // the validators can refer to them

    let has_map_level_rules = !map_level_rules.is_empty();

    let map_level_context_tokens = has_map_level_rules
      .then(|| self.field_context_tokens(self.field_kind, self.field_context_ident));

    let has_keys_rules = !keys_rules.is_empty();
    let has_values_rules = !values_rules.is_empty();

    let has_loop = has_keys_rules || has_values_rules;

    let keys_context_tokens = has_keys_rules.then(|| {
      let keys_proto_type = self.map_keys_type.unwrap_or_else(|| {
        panic!(
          "Could not get the map key type for field {}",
          self.full_name
        )
      });

      self.field_context_tokens(
        FieldKind::MapKey(keys_proto_type.into()),
        self.map_key_context_ident,
      )
    });

    let values_context_tokens = has_values_rules.then(|| {
      let values_proto_type = self.map_values_type.unwrap_or_else(|| {
        panic!(
          "Could not get the map value type for field {}",
          self.full_name
        )
      });

      self.field_context_tokens(
        FieldKind::MapValue(values_proto_type.into()),
        self.map_value_context_ident,
      )
    });

    let loop_tokens = has_loop.then(|| {
      let field_ident = &self.item_rust_ident;
      let key_ident = &self.map_key_ident;
      let map_value_ident = &self.map_value_ident;

      quote! {
        for (#key_ident, #map_value_ident) in self.#field_ident.iter() {
          #keys_context_tokens
          #keys_rules

          #values_context_tokens
          #values_rules
        }
      }
    });

    tokens.extend(quote! {
      #map_level_context_tokens
      #map_level_rules

      #loop_tokens
    });
  }

  pub fn field_context_ident(&self) -> &Ident {
    // These must be different because a map might have validators for keys, values and
    // for the map as a whole, with separate contexts

    match self.field_kind {
      FieldKind::Single(_) | FieldKind::Repeated(_) | FieldKind::Map(_) => self.field_context_ident,
      FieldKind::RepeatedItem(_) => self.vec_item_context_ident,
      FieldKind::MapKey(_) => self.map_key_context_ident,
      FieldKind::MapValue(_) => self.map_value_context_ident,
    }
  }

  pub fn to_map_key(&'_ self, field_type: FieldType) -> ValidationData<'_> {
    let mut key_validation_data = self.clone();
    key_validation_data.field_kind = FieldKind::MapKey(field_type);
    key_validation_data.value_ident = OnceCell::new();

    key_validation_data
  }

  pub fn to_map_value(&'_ self, field_type: FieldType) -> ValidationData<'_> {
    let mut value_validation_data = self.clone();
    value_validation_data.field_kind = FieldKind::MapValue(field_type);
    value_validation_data.value_ident = OnceCell::new();

    value_validation_data
  }

  pub fn to_repeated_item(&'_ self, field_desc: &FieldDescriptor) -> ValidationData<'_> {
    let mut items_validation_data = self.clone();
    items_validation_data.field_kind = FieldKind::RepeatedItem(get_field_type(field_desc));
    items_validation_data.value_ident = OnceCell::new();

    items_validation_data
  }

  pub fn aggregate_vec_rules(&self, tokens: &mut TokenStream2, rules_data: &RepeatedValidator) {
    let RepeatedValidator {
      vec_level_rules,
      items_rules,
    } = rules_data;

    // Validators are already aggregated here, we just inject the context for the
    // vec or its items, and the surrounding loop

    let has_loop = !items_rules.is_empty();
    let has_vec_level_rules = !vec_level_rules.is_empty();

    let vec_level_field_context = has_vec_level_rules
      .then(|| self.field_context_tokens(self.field_kind, self.field_context_ident));

    let loop_tokens = has_loop.then(|| {
      let field_ident = &self.item_rust_ident;
      let index_ident = &self.index_ident;
      let item_ident = &self.item_ident;

      let items_context_tokens = self.field_context_tokens(
        FieldKind::RepeatedItem(self.field_kind.inner_type()),
        self.vec_item_context_ident,
      );

      quote! {
        for (#index_ident, #item_ident) in self.#field_ident.iter().enumerate() {
          #items_context_tokens
          #items_rules
        }
      }
    });

    tokens.extend(quote! {
      #vec_level_field_context
      #vec_level_rules

      #loop_tokens
    });
  }

  pub fn get_const_validator<T>(&self, tokens: &mut TokenStream2, rule: ConstRule<T>)
  where
    T: ToTokens,
  {
    let proto_type_name = self.field_kind.inner_type().name();
    let func_ident = format_ident!("{proto_type_name}_const");
    let field_context_ident = self.field_context_ident();
    let value_ident = self.value_ident();

    let ConstRule { val, error_message } = rule;

    let expr = quote! { ::protocheck::validators::constants::#func_ident(&#field_context_ident, #value_ident, #val, #error_message) };

    self.get_validator_tokens(tokens, &expr);
  }

  pub fn get_message_field_validator_tokens(
    &self,
    tokens: &mut TokenStream2,
    field_kind: FieldKind,
  ) {
    let Self {
      parent_messages_ident,
      violations_ident,
      ..
    } = self;

    let field_proto_name = &self.proto_name;
    let field_tag = self.tag;
    let field_proto_type: ProtoType = self.field_kind.inner_type().into();

    // Not using self.value_ident() on purpose here
    // because we don't care about box or refs here, it's always a ref anyway
    let value_ident = match field_kind {
      FieldKind::RepeatedItem(_) => self.item_ident,
      FieldKind::MapKey(_) => self.map_key_ident,
      FieldKind::MapValue(_) => self.map_value_ident,
      _ => &format_ident!("val"),
    };

    // These are not part of the FieldContext but of FieldPathElement
    // so they need to be cast as integers
    let field_key_type = self
      .map_keys_type
      .map_or(quote! { None }, |k| quote! { Some(#k as i32) });

    let field_value_type = self
      .map_values_type
      .map_or(quote! { None }, |v| quote! { Some(#v as i32) });

    let subscript_tokens = self.subscript_tokens(field_kind);

    let field_path_element_tokens = quote! {
      ::protocheck::types::protovalidate::FieldPathElement {
        field_name: Some(#field_proto_name.to_string()),
        field_number: Some(#field_tag as i32),
        field_type: Some(#field_proto_type as i32),
        key_type: #field_key_type,
        value_type: #field_value_type,
        subscript: #subscript_tokens,
      };
    };

    tokens.extend(quote! {
      let current_nested_field_element = #field_path_element_tokens;

      #parent_messages_ident.push(current_nested_field_element);
      #value_ident.nested_validate(#parent_messages_ident, #violations_ident);
      #parent_messages_ident.pop();
    });
  }

  pub fn get_required_validation_tokens(&self) -> Option<TokenStream2> {
    self.is_required.then(|| {
      let field_context_tokens = self.field_context_tokens(self.field_kind, self.field_context_ident);
      let field_context_ident = self.field_context_ident();
      let violations_ident = &self.violations_ident;

      quote! {
        #field_context_tokens
        let required_violation = ::protocheck::validators::required::required(&#field_context_ident);
        #violations_ident.push(required_violation);
      }
    })
  }

  pub fn get_required_only_validator(&self, tokens: &mut TokenStream2) {
    let item_rust_ident = &self.item_rust_ident;
    let required_validation_tokens = self.get_required_validation_tokens();

    tokens.extend(quote! {
      if self.#item_rust_ident.is_none() {
        #required_validation_tokens
      }
    });
  }

  pub fn get_aggregated_validator_tokens(&self, validators: TokenStream2) -> TokenStream2 {
    let field_context_tokens = self.field_context_tokens(self.field_kind, self.field_context_ident);
    let required_check = self.get_required_validation_tokens();
    let field_ident = self.item_rust_ident;

    if self.is_option() {
      let match_kind = if self.field_kind.is_copy() {
        quote! { self.#field_ident }
      } else {
        quote! { self.#field_ident.as_ref() }
      };

      quote! {
        match #match_kind {
          Some(val) => {
            #field_context_tokens
            #validators
          },
          None => { #required_check }
        };
      }
    } else {
      let validation_tokens = quote! {
        #field_context_tokens
        #validators
      };

      if matches!(self.ignore, Ignore::IfZeroValue) && !self.is_in_oneof {
        self.wrap_with_default_value_check(validation_tokens)
      } else {
        validation_tokens
      }
    }
  }

  pub fn wrap_with_default_value_check(&self, validators: TokenStream2) -> TokenStream2 {
    let value_ident = self.value_ident();

    let default_check = match self.field_kind.inner_type() {
      FieldType::Bytes | FieldType::String => quote! { !#value_ident.is_empty() },
      FieldType::Bool => quote! { #value_ident },
      FieldType::Float | FieldType::Double => quote! { #value_ident != 0.0 },
      _ => quote! { #value_ident != 0 },
    };

    quote! {
      if #default_check {
        #validators
      }
    }
  }

  pub fn get_validator_tokens(
    &self,
    tokens: &mut TokenStream2,
    validator_expression_tokens: &TokenStream2,
  ) {
    let violations_ident = &self.violations_ident;

    tokens.extend(quote! {
      match #validator_expression_tokens {
        Ok(_) => {}
        Err(v) => #violations_ident.push(v)
      };
    });
  }

  pub fn is_option(&self) -> bool {
    self.is_optional && !self.is_in_oneof
  }

  pub fn subscript_tokens(&self, field_kind: FieldKind) -> TokenStream2 {
    match field_kind {
      FieldKind::RepeatedItem(_) => {
        let index_ident = self.index_ident;
        quote! { Some(::protocheck::types::protovalidate::field_path_element::Subscript::Index(#index_ident as u64)) }
      }

      FieldKind::MapKey(_) | FieldKind::MapValue(_) => {
        if let Some(key_type) = self.map_keys_type {
          let key_subscript_tokens = generate_key_subscript(&key_type, self.map_key_ident);
          quote! { Some(#key_subscript_tokens) }
        } else {
          let error = Error::new(
            self.field_span,
            "Map key type is missing during macro expansion.",
          )
          .to_compile_error();

          quote! { #error }
        }
      }
      _ => quote! { None },
    }
  }

  pub fn value_ident(&self) -> &TokenStream2 {
    let Self {
      item_rust_ident,
      map_key_ident: key_ident,
      map_value_ident,
      item_ident,
      ..
    } = self;

    self.value_ident.get_or_init(|| {
      if self.is_boxed {
        return quote! { *val };
      }
      let is_copy = self.field_kind.is_copy();

      match self.field_kind {
        FieldKind::RepeatedItem(_) => {
          if is_copy {
            quote! { #item_ident.clone() }
          } else {
            quote! { #item_ident }
          }
        }
        FieldKind::MapKey(_) => {
          if is_copy {
            quote! { #key_ident.clone() }
          } else {
            quote! { #key_ident }
          }
        }
        FieldKind::MapValue(_) => {
          if is_copy {
            quote! { #map_value_ident.clone() }
          } else {
            quote! { #map_value_ident }
          }
        }
        FieldKind::Repeated(_) => quote! { self.#item_rust_ident },
        FieldKind::Map(_) => quote! { self.#item_rust_ident },
        FieldKind::Single(_) => {
          let base_ident = if self.is_optional || self.is_in_oneof {
            quote! { val }
          } else {
            quote! { self.#item_rust_ident }
          };

          if !is_copy && !self.is_option() {
            quote! { &#base_ident }
          } else {
            base_ident
          }
        }
      }
    })
  }
}

fn generate_key_subscript(key_proto_type: &ProtoType, key_ident: &Ident) -> TokenStream2 {
  let subscript_path = quote! { ::protocheck::types::protovalidate::field_path_element::Subscript };

  match key_proto_type {
    ProtoType::String => quote! { #subscript_path::StringKey(#key_ident.clone().into()) },

    ProtoType::Uint64 | ProtoType::Uint32 | ProtoType::Fixed64 | ProtoType::Fixed32 => quote! { #subscript_path::UintKey(#key_ident.clone().into()) },

    ProtoType::Int64 | ProtoType::Int32 | ProtoType::Sfixed64 | ProtoType::Sfixed32 | ProtoType::Sint64 | ProtoType::Sint32 => quote! { #subscript_path::IntKey(#key_ident.clone().into()) },

    ProtoType::Bool => quote! { #subscript_path::BoolKey(#key_ident) },

    _ => {
      Error::new_spanned(
        key_ident,
        format!("Unsupported Protobuf type {key_proto_type:?} for map key. Only integer, string, and bool types are allowed."
      ))
        .into_compile_error()
        .into_token_stream()
    }
  }
}
