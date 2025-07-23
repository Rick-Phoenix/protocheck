use cel_interpreter::{Context, Program};
use serde::Serialize;

#[derive(Serialize)]
struct Test {
  first_name: String,
  last_name: Nested,
}

#[derive(Serialize)]
struct Nested {
  field: String,
}

fn main() {
  let person = Test {
    first_name: String::from("io"),
    last_name: Nested {
      field: String::from("io"),
    },
  };
  let program = Program::compile("this.last_name.field").unwrap();
  let mut context = Context::default();

  context.add_variable("this", person).unwrap();

  let value = program.execute(&context).unwrap();
  println!("{:?}", value);
}
