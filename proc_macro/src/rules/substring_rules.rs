use crate::*;

pub struct SubstringRule {
  pub val_tokens: TokenStream2,
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

pub trait RulesWithSubstring {
  fn substring_rules(&self) -> SubstringRules;
}

impl RulesWithSubstring for StringRules {
  fn substring_rules(&self) -> SubstringRules {
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

impl RulesWithSubstring for BytesRules {
  fn substring_rules(&self) -> SubstringRules {
    let contains = self.contains.as_ref().map(|v| SubstringRule {
      val_tokens: LitByteStr::new(v, Span::call_site()).to_token_stream(),
      error_message: format!("must contain {}", v.escape_ascii()),
    });
    let prefix = self.prefix.as_ref().map(|v| SubstringRule {
      val_tokens: LitByteStr::new(v, Span::call_site()).to_token_stream(),
      error_message: format!("must start with {}", v.escape_ascii()),
    });
    let suffix = self.suffix.as_ref().map(|v| SubstringRule {
      val_tokens: LitByteStr::new(v, Span::call_site()).to_token_stream(),
      error_message: format!("must end with {}", v.escape_ascii()),
    });

    SubstringRules {
      contains,
      not_contains: None,
      prefix,
      suffix,
    }
  }
}
