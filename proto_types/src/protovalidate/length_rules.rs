use crate::protovalidate::{BytesRules, MapRules, RepeatedRules, StringRules};

pub enum LengthRulesTarget {
  String,
  Bytes,
  RepeatedItems,
  MapPairs,
}

pub enum LengthRulesKind {
  Len,
  LenBytes,
  RepeatedItems,
  MapPairs,
}

pub struct LengthRules {
  pub len: Option<u64>,
  pub min_len: Option<u64>,
  pub max_len: Option<u64>,
  pub target: LengthRulesTarget,
  pub kind: LengthRulesKind,
}

impl LengthRules {
  pub fn name(&self) -> &'static str {
    match self.target {
      LengthRulesTarget::String => "string",
      LengthRulesTarget::Bytes => "bytes",
      LengthRulesTarget::RepeatedItems => "repeated",
      LengthRulesTarget::MapPairs => "maps",
    }
  }

  pub fn has_rule(&self) -> bool {
    self.len.is_some() || self.min_len.is_some() || self.max_len.is_some()
  }

  pub fn unit(&self) -> &str {
    match self.target {
      LengthRulesTarget::String => "character",
      LengthRulesTarget::Bytes => "byte",
      LengthRulesTarget::RepeatedItems => "item",
      LengthRulesTarget::MapPairs => "key-value pair",
    }
  }

  pub fn len_name(&self) -> &str {
    match self.kind {
      LengthRulesKind::Len => "len",
      LengthRulesKind::LenBytes => "len_bytes",
      _ => "",
    }
  }

  pub fn min_len_name(&self) -> &str {
    match self.kind {
      LengthRulesKind::Len => "min_len",
      LengthRulesKind::LenBytes => "min_bytes",
      LengthRulesKind::RepeatedItems => "min_items",
      LengthRulesKind::MapPairs => "min_pairs",
    }
  }

  pub fn max_len_name(&self) -> &str {
    match self.kind {
      LengthRulesKind::Len => "max_len",
      LengthRulesKind::LenBytes => "max_bytes",
      LengthRulesKind::RepeatedItems => "max_items",
      LengthRulesKind::MapPairs => "max_pairs",
    }
  }

  pub fn validate(self) -> Result<LengthRules, String> {
    let len = self.len;
    let min_len = self.min_len;
    let max_len = self.max_len;

    if len.is_some() && (min_len.is_some() || max_len.is_some()) {
      return Err(format!(
        "{} cannot be used with {} or {}",
        self.len_name(),
        self.min_len_name(),
        self.max_len_name()
      ));
    }

    if let Some(min) = min_len
      && let Some(max) = max_len
        && min > max {
          return Err(format!("{} cannot be larger than {}", self.min_len_name(), self.max_len_name()));
        }

    Ok(self)
  }
}

impl RepeatedRules {
  pub fn length_rules(&self) -> Result<LengthRules, String> {
    let min_len = self.min_items;
    let max_len = self.max_items;

    LengthRules {
      len: None,
      min_len,
      max_len,
      target: LengthRulesTarget::RepeatedItems,
      kind: LengthRulesKind::RepeatedItems,
    }
    .validate()
  }
}

impl MapRules {
  pub fn length_rules(&self) -> Result<LengthRules, String> {
    let min_len = self.min_pairs;
    let max_len = self.max_pairs;

    LengthRules {
      len: None,
      min_len,
      max_len,
      target: LengthRulesTarget::MapPairs,
      kind: LengthRulesKind::MapPairs,
    }
    .validate()
  }
}

impl BytesRules {
  pub fn length_rules(&self) -> Result<LengthRules, String> {
    let len = self.len;
    let min_len = self.min_len;
    let max_len = self.max_len;

    LengthRules {
      len,
      min_len,
      max_len,
      target: LengthRulesTarget::Bytes,
      kind: LengthRulesKind::Len,
    }
    .validate()
  }
}

impl StringRules {
  pub fn length_rules(&self) -> Result<LengthRules, String> {
    let len = self.len;
    let min_len = self.min_len;
    let max_len = self.max_len;

    LengthRules {
      len,
      min_len,
      max_len,
      target: LengthRulesTarget::String,
      kind: LengthRulesKind::Len,
    }
    .validate()
  }

  pub fn bytes_length_rules(&self) -> Result<LengthRules, String> {
    let len = self.len_bytes;
    let min_len = self.min_bytes;
    let max_len = self.max_bytes;

    LengthRules {
      len,
      min_len,
      max_len,
      target: LengthRulesTarget::String,
      kind: LengthRulesKind::LenBytes,
    }
    .validate()
  }
}
