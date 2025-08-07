use std::{collections::HashSet, fmt::Debug, hash::Hash};

use proto_types::{Any, Duration};

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation, in_rules::get_in_rule_path,
    not_in_rules::get_not_in_rule_path,
  },
};

pub fn duration_in_list(
  field_context: &FieldContext,
  value: Duration,
  target: &'static HashSet<Duration>,
) -> Result<(), Violation> {
  let check = target.contains(&value);
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, d) in target.iter().enumerate() {
      list_str.push_str(&d.display_full());

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("has to be one of these values: [ {} ]", list_str);
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn duration_not_in_list(
  field_context: &FieldContext,
  value: Duration,
  target: &'static HashSet<Duration>,
) -> Result<(), Violation> {
  let check = !target.contains(&value);
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, d) in target.iter().enumerate() {
      list_str.push_str(&d.display_full());

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("cannot be one of these values: [ {} ]", list_str);
    Err(create_not_in_list_violation(
      field_context,
      &values_list_string,
    ))
  }
}

pub fn string_in_list(
  field_context: &FieldContext,
  value: &str,
  target: &'static HashSet<&'static str>,
) -> Result<(), Violation> {
  let check = target.contains(value);
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, s) in target.iter().enumerate() {
      list_str.push_str(s);

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("has to be one of these values: [ {} ]", list_str);
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn string_not_in_list(
  field_context: &FieldContext,
  value: &str,
  target: &'static HashSet<&'static str>,
) -> Result<(), Violation> {
  let check = !target.contains(value);
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, s) in target.iter().enumerate() {
      list_str.push_str(s);

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("cannot be one of these values: [ {} ]", list_str);
    Err(create_not_in_list_violation(
      field_context,
      &values_list_string,
    ))
  }
}

pub fn any_in_list(
  field_context: &FieldContext,
  value: &Any,
  target: &'static HashSet<&'static str>,
) -> Result<(), Violation> {
  let check = target.contains(value.type_url.as_str());
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, s) in target.iter().enumerate() {
      list_str.push_str(s);

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!(
      "the type url has to be one of these values: [ {} ]",
      list_str
    );
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn any_not_in_list(
  field_context: &FieldContext,
  value: &Any,
  target: &'static HashSet<&'static str>,
) -> Result<(), Violation> {
  let check = !target.contains(value.type_url.as_str());
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, s) in target.iter().enumerate() {
      list_str.push_str(s);

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!(
      "the type url cannot be one of these values: [ {} ]",
      list_str
    );
    Err(create_not_in_list_violation(
      field_context,
      &values_list_string,
    ))
  }
}

pub fn f32_in_list(
  field_context: &FieldContext,
  value: f32,
  target: &'static HashSet<u32>,
) -> Result<(), Violation> {
  let check = target.contains(&value.to_bits());
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, n) in target.iter().enumerate() {
      list_str.push_str(&f32::from_bits(*n).to_string());

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("has to be one of these values: [ {} ]", list_str);
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn f32_not_in_list(
  field_context: &FieldContext,
  value: f32,
  target: &'static HashSet<u32>,
) -> Result<(), Violation> {
  let check = !target.contains(&value.to_bits());
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, n) in target.iter().enumerate() {
      list_str.push_str(&f32::from_bits(*n).to_string());

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("cannot be one of these values: [ {} ]", list_str);
    Err(create_not_in_list_violation(
      field_context,
      &values_list_string,
    ))
  }
}

pub fn f64_in_list(
  field_context: &FieldContext,
  value: f64,
  target: &'static HashSet<u64>,
) -> Result<(), Violation> {
  let check = target.contains(&value.to_bits());
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, n) in target.iter().enumerate() {
      list_str.push_str(&f64::from_bits(*n).to_string());

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("has to be one of these values: [ {} ]", list_str);
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn f64_not_in_list(
  field_context: &FieldContext,
  value: f64,
  target: &'static HashSet<u64>,
) -> Result<(), Violation> {
  let check = !target.contains(&value.to_bits());
  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, n) in target.iter().enumerate() {
      list_str.push_str(&f64::from_bits(*n).to_string());

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("cannot be one of these values: [ {} ]", list_str);
    Err(create_not_in_list_violation(
      field_context,
      &values_list_string,
    ))
  }
}

fn create_in_list_violation(field_context: &FieldContext, error_message: &str) -> Violation {
  let (type_name, violation_path) = get_in_rule_path(field_context.field_kind.inner_type())
    .expect("Could not find 'in_list' rule path");
  let rule_id = format!("{}.in", type_name);

  create_violation(field_context, violation_path, &rule_id, error_message)
}

pub fn in_list<T>(
  field_context: &FieldContext,
  value: T,
  target: &'static HashSet<T>,
) -> Result<(), Violation>
where
  T: Eq + Debug + Hash,
{
  let check = target.contains(&value);

  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, v) in target.iter().enumerate() {
      list_str.push_str(&format!("{:?}", v));

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("has to be one of these values: [ {} ]", list_str);
    Err(create_in_list_violation(field_context, &values_list_string))
  }
}

pub fn not_in_list<T>(
  field_context: &FieldContext,
  value: T,
  target: &'static HashSet<T>,
) -> Result<(), Violation>
where
  T: Eq + Hash + Debug,
{
  let check = !target.contains(&value);

  if check {
    Ok(())
  } else {
    let mut list_str = String::new();

    for (i, v) in target.iter().enumerate() {
      list_str.push_str(&format!("{:?}", v));

      if i != target.len() - 1 {
        list_str.push_str(", ");
      }
    }

    let values_list_string = format!("cannot be one of these values: [ {} ]", list_str);
    Err(create_not_in_list_violation(
      field_context,
      &values_list_string,
    ))
  }
}

fn create_not_in_list_violation(field_context: &FieldContext, error_message: &str) -> Violation {
  let (type_name, violation_path) = get_not_in_rule_path(field_context.field_kind.inner_type())
    .expect("Could not find 'not_in_list' rule path");
  let rule_id = format!("{}.not_in", type_name);

  create_violation(field_context, violation_path, &rule_id, error_message)
}
