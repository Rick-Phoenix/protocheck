use quote::{quote, ToTokens};

use crate::{Duration, TokenStream2};

impl ToTokens for Duration {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let seconds = self.seconds;
    let nanos = self.nanos;

    tokens.extend(quote! {
      protocheck::types::protobuf::Duration {
        seconds: #seconds,
        nanos: #nanos,
      }
    });
  }
}
