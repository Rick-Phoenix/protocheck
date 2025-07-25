// use syn::Error;
//
// use crate::Span2;
//
// pub trait ResultsAgg<T> {
//   fn agg_results_spanned(self, results: &mut Vec<T>, errors: &mut Vec<Error>, span: Span2);
//   fn agg_results(self, results: &mut Vec<T>, errors: &mut Vec<Error>);
// }
//
// impl<T> ResultsAgg<T> for Result<Vec<T>, Vec<Error>> {
//   fn agg_results_spanned(self, results: &mut Vec<T>, errors: &mut Vec<Error>, span: Span2) {
//     match self {
//       Ok(value) => {
//         results.extend(value);
//       }
//       Err(e) => {
//         errors.extend(e);
//       }
//     }
//   }
//
//   fn agg_results(self, results: &mut Vec<T>, errors: &mut Vec<Error>) {
//     match self {
//       Ok(value) => {
//         results.extend(value);
//       }
//       Err(e) => {
//         errors.extend(e);
//       }
//     }
//   }
// }
//
// pub trait ResultAgg<T, E> {
//   fn agg_result_spanned(self, results: &mut Vec<T>, errors: &mut Vec<Error>, span: Span2);
//   fn agg_result(self, results: &mut Vec<T>, errors: &mut Vec<Error>);
// }
//
// impl<T, E> ResultAgg<T, E> for Result<T, E>
// where
//   E: std::fmt::Display,
// {
//   fn agg_result_spanned(self, results: &mut Vec<T>, errors: &mut Vec<Error>, span: Span2) {
//     match self {
//       Ok(value) => {
//         results.push(value);
//       }
//       Err(e) => {
//         errors.push(Error::new(span, e.to_string()));
//       }
//     }
//   }
//
//   fn agg_result(self, results: &mut Vec<T>, errors: &mut Vec<Error>) {
//     match self {
//       Ok(value) => {
//         results.push(value);
//       }
//       Err(e) => {
//         errors.push(Error::new(Span2::call_site(), e.to_string()));
//       }
//     }
//   }
// }
//
// pub trait ResultErrAgg<T> {
//   fn agg_result_spanned(self, results: &mut Vec<T>, errors: &mut Vec<Error>, span: Span2);
//   fn agg_result(self, results: &mut Vec<T>, errors: &mut Vec<Error>);
// }
//
// impl<T> ResultErrAgg<T> for Result<T, Vec<Error>> {
//   fn agg_result_spanned(self, results: &mut Vec<T>, errors: &mut Vec<Error>, span: Span2) {
//     match self {
//       Ok(value) => {
//         results.push(value);
//       }
//       Err(e) => {
//         errors.extend(e);
//       }
//     }
//   }
//
//   fn agg_result(self, results: &mut Vec<T>, errors: &mut Vec<Error>) {
//     match self {
//       Ok(value) => {
//         results.push(value);
//       }
//       Err(e) => {
//         errors.extend(e);
//       }
//     }
//   }
// }
