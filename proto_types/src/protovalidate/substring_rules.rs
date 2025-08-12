use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::LitByteStr;

use crate::protovalidate::{containing_rules::format_bytes, BytesRules, StringRules};

pub struct SubstringRule {
  pub val_tokens: TokenStream,
  pub error_message: String,
}

pub struct SubstringRules {
  pub contains: Option<SubstringRule>,
  pub not_contains: Option<SubstringRule>,
  pub prefix: Option<SubstringRule>,
  pub suffix: Option<SubstringRule>,
}

impl SubstringRules {
  pub fn has_rule(&self) -> bool {
    self.contains.is_some()
      || self.not_contains.is_some()
      || self.prefix.is_some()
      || self.suffix.is_some()
  }
}

impl StringRules {
  pub fn substring_rules(&self) -> SubstringRules {
    let contains = self.contains.as_ref().map(|v| SubstringRule {
      val_tokens: v.to_token_stream(),
      error_message: format!("must contain the substring '{}'", v),
    });
    let not_contains = self.not_contains.as_ref().map(|v| SubstringRule {
      val_tokens: v.to_token_stream(),
      error_message: format!("must not contain the substring '{}'", v),
    });

    let prefix = self.prefix.as_ref().map(|v| SubstringRule {
      val_tokens: v.to_token_stream(),
      error_message: format!("must start with '{}'", v),
    });
    let suffix = self.suffix.as_ref().map(|v| SubstringRule {
      val_tokens: v.to_token_stream(),
      error_message: format!("must end with '{}'", v),
    });

    SubstringRules {
      contains,
      not_contains,
      prefix,
      suffix,
    }
  }
}

impl BytesRules {
  pub fn substring_rules(&self) -> SubstringRules {
    let contains = self.contains.as_ref().map(|v| SubstringRule {
      val_tokens: LitByteStr::new(v, Span::call_site()).to_token_stream(),
      error_message: format!("must contain {}", format_bytes(v)),
    });
    let prefix = self.prefix.as_ref().map(|v| SubstringRule {
      val_tokens: LitByteStr::new(v, Span::call_site()).to_token_stream(),
      error_message: format!("must start with {}", format_bytes(v)),
    });
    let suffix = self.suffix.as_ref().map(|v| SubstringRule {
      val_tokens: LitByteStr::new(v, Span::call_site()).to_token_stream(),
      error_message: format!("must end with {}", format_bytes(v)),
    });

    SubstringRules {
      contains,
      not_contains: None,
      prefix,
      suffix,
    }
  }
}
