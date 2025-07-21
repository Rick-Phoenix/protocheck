// #[macro_export]
// macro_rules! wrap_loop {
//   ($field_data:expr,
//   $collection_exrp:expr,
//   $idx_name:ident,
//   $item_name:ident,
//   {$($inner_logic:tt)* }
// ) => {
//   let field_data = $field_data.clone();
//   for ($idx_name, $item_name) in $collection_exrp.iter().enumerate() {
//     $($inner_logic)*
//   }
// }
// }
