use crate::*;

static PROST_KEYWORDS_RENAMED_WITH_R: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
  [
    "as", "break", "const", "continue", "else", "enum", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "static", "struct",
    "trait", "true", "type", "unsafe", "use", "where", "while", "abstract", "become", "box", "do",
    "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "async",
    "await",
  ]
  .iter()
  .copied()
  .collect()
});

static PROST_KEYWORDS_RENAMED_WITH_UNDERSCORE: [&str; 4] = ["crate", "extern", "super", "self"];

pub fn proto_name_to_rust_name(proto_name: &str) -> Cow<'_, str> {
  if PROST_KEYWORDS_RENAMED_WITH_UNDERSCORE.contains(&proto_name) {
    Cow::Owned(format!("{}_", proto_name))
  } else if PROST_KEYWORDS_RENAMED_WITH_R.contains(proto_name) {
    Cow::Owned(format!("r#{}", proto_name))
  } else {
    Cow::Borrowed(proto_name)
  }
}

pub fn proto_name_to_rust_ident(proto_name: &str) -> Ident {
  if PROST_KEYWORDS_RENAMED_WITH_UNDERSCORE.contains(&proto_name) {
    format_ident!("{}_", proto_name)
  } else if PROST_KEYWORDS_RENAMED_WITH_R.contains(proto_name) {
    format_ident!("r#{}", proto_name)
  } else {
    format_ident!("{}", proto_name)
  }
}
