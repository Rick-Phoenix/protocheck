use crate::*;

pub fn new_ident(name: &str) -> Ident {
  Ident::new(name, Span::call_site())
}
