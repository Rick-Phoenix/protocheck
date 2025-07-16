use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr, Type}; // Import necessary syn items

// This macro attribute will be applied to a struct.
// It will add a `debug_print()` method to the struct.
#[proc_macro_attribute]
pub fn debug_print_fields(_attr: TokenStream, item: TokenStream) -> TokenStream {
  // 1. Parse the input `item` as a Rust struct definition.
  // If it's not a struct, `parse_macro_input!` will generate a compile error.
  let input_struct = parse_macro_input!(item as syn::ItemStruct);

  // Extract the name (identifier) of the struct
  let struct_name = &input_struct.ident;

  // Prepare a vector to hold the quote! blocks for each field's debug print statement
  let mut field_print_statements = Vec::new();

  // Iterate over the fields of the struct
  for field in &input_struct.fields {
    // Get the field's name (Ident). If it's a tuple struct field (like `struct Foo(usize)`), it won't have a name.
    let field_name = match &field.ident {
      Some(ident) => ident.clone(), // Get the field's identifier
      None => {
        // If it's a tuple struct field (e.g., struct MyTuple(u32, bool);)
        // We'll skip it for this simple example, or you could use field.index if needed.
        continue;
      }
    };

    // Construct the quote! block for printing this field
    // We use stringify! to get the field's name as a string literal at compile time.
    // #field_name directly interpolates the `syn::Ident` into the generated code.
    field_print_statements.push(quote! {
        println!("  {}: {:?}", stringify!(#field_name), self.#field_name);
    });
  }

  let output = quote! {
      // Include the original struct definition.
      // It's crucial to always return the original item you're decorating,
      // otherwise it will be removed from the compiled code!
      #input_struct

      impl #struct_name {
          /// Prints the name of the struct and the debug representation of its fields.
          /// Requires the struct to derive `Debug` if its fields don't have Display impls.
          pub fn debug_print(&self) {
              println!("--- Struct: {} ---", stringify!(#struct_name));
              // Interpolate all the collected field print statements
              #(#field_print_statements)*
              println!("--------------------");
          }
      }
  };

  // 3. Convert the generated `proc_macro2::TokenStream` into `proc_macro::TokenStream`
  // and return it.
  output.into()
}

#[proc_macro_derive(Hello)] // The name in derive(...) matches the macro function name
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // 1. Parse the input `item` as a DeriveInput.
  // DeriveInput is a special syn struct designed for parsing `#[derive]` input.
  let input_ast = parse_macro_input!(input as syn::DeriveInput);

  // Extract the name (identifier) of the struct or enum
  let name = &input_ast.ident;

  // We'll also need to handle generics so the impl works for generic structs.
  // syn::Generics provides fields for type parameters, lifetimes, and const parameters.
  let (impl_generics, ty_generics, where_clause) = input_ast.generics.split_for_impl();

  // 2. Generate the output `TokenStream`.
  // This will be an `impl` block for the `Hello` trait for the struct.
  let output = quote! {
      // Implement the `Hello` trait for the given struct `name`.
      // We use `impl_generics` and `ty_generics` to ensure the impl applies correctly
      // if the original struct is generic (e.g., `struct MyGeneric<T>`).
    impl #impl_generics macro_impl::Hello for #name #ty_generics #where_clause {
          fn hello(&self) {
              // `stringify!(#name)` turns the struct's identifier into a string literal.
              println!("Hello from {}!", stringify!(#name));
          }
      }
  };

  // 3. Convert and return the generated TokenStream.
  output.into()
}
