#![allow(clippy::bool_assert_comparison)]

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

fn main() {}

#[cfg(test)]
mod tests;
