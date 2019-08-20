use super::{
  super::{CompilationError, Error, Result},
  JSONError,
};
use crate::{token::Numeric, ParserError};
use regex::Regex;
use serde_json::{self, Value};

/// Validates a JSON value against a given Perl-Compatible regex controller
pub fn validate_pcre_control(controller: &str, value: &Value) -> Result {
  match value {
    Value::String(s) => {
      // Text strings must follow JSON string conventions per
      // https://www.rfc-editor.org/rfc/rfc8610.html#section-3.1. Since the pcre
      // control operates on text strings, it must be unescaped before being
      // consumed by the regex crate.
      let re = Regex::new(
        serde_json::from_str::<Value>(&format!("\"{}\"", controller))
          .map_err(|e| Error::Syntax(e.to_string()))?
          .as_str()
          .ok_or_else(|| Error::Syntax("Malformed regex".into()))?,
      )
      .map_err(|e| Error::Compilation(CompilationError::CDDL(ParserError::REGEX(e))))?;

      if re.is_match(s) {
        return Ok(());
      }

      Err(
        JSONError {
          expected_memberkey: None,
          expected_value: format!("text .pcre {}", controller),
          actual_memberkey: None,
          actual_value: value.clone(),
        }
        .into(),
      )
    }
    _ => Err(
      JSONError {
        expected_memberkey: None,
        expected_value: format!("text .pcre {:?}", controller),
        actual_memberkey: None,
        actual_value: value.clone(),
      }
      .into(),
    ),
  }
}

/// Validates whether or not a JSON value is less than a given numeric
/// controller
pub fn validate_lt_control(controller: Numeric, value: &Value) -> Result {
  match value {
    Value::Number(n) => match controller {
      Numeric::INT(i) => match n.as_i64() {
        Some(ni) if ni < i as i64 => Ok(()),
        _ => Err(
          JSONError {
            expected_memberkey: None,
            expected_value: format!("expected int < {}", i),
            actual_memberkey: None,
            actual_value: value.clone(),
          }
          .into(),
        ),
      },
      Numeric::UINT(ui) => match n.as_u64() {
        Some(uin) if uin < ui as u64 => Ok(()),
        _ => Err(
          JSONError {
            expected_memberkey: None,
            expected_value: format!("expected uint < {}", ui),
            actual_memberkey: None,
            actual_value: value.clone(),
          }
          .into(),
        ),
      },
      Numeric::FLOAT(f) => match n.as_f64() {
        Some(fv) if fv < f => Ok(()),
        _ => Err(
          JSONError {
            expected_memberkey: None,
            expected_value: format!("expected float < {}", f),
            actual_memberkey: None,
            actual_value: value.clone(),
          }
          .into(),
        ),
      },
    },
    _ => Err(Error::Syntax(format!(
      ".lt control can only be used against numeric values. Got {}",
      value
    ))),
  }
}

/// Validates whether or not a JSON value is greater than a given numeric
/// controller
pub fn validate_gt_control(controller: Numeric, value: &Value) -> Result {
  match value {
    Value::Number(n) => match controller {
      Numeric::INT(i) => match n.as_i64() {
        Some(ni) if ni > i as i64 => Ok(()),
        _ => Err(
          JSONError {
            expected_memberkey: None,
            expected_value: format!("expected int > {}", i),
            actual_memberkey: None,
            actual_value: value.clone(),
          }
          .into(),
        ),
      },
      Numeric::UINT(ui) => match n.as_u64() {
        Some(uin) if uin > ui as u64 => Ok(()),
        _ => Err(
          JSONError {
            expected_memberkey: None,
            expected_value: format!("expected uint > {}", ui),
            actual_memberkey: None,
            actual_value: value.clone(),
          }
          .into(),
        ),
      },
      Numeric::FLOAT(f) => match n.as_f64() {
        Some(fv) if fv > f => Ok(()),
        _ => Err(
          JSONError {
            expected_memberkey: None,
            expected_value: format!("expected float > {}", f),
            actual_memberkey: None,
            actual_value: value.clone(),
          }
          .into(),
        ),
      },
    },
    _ => Err(Error::Syntax(format!(
      ".gt control can only be used against numeric values. Got {}",
      value
    ))),
  }
}

#[cfg(test)]
mod tests {
  use super::super::{validate_json_from_str, Result};

  #[test]
  fn validate_pcre_control() -> Result {
    let json_input = r#""N1@CH57HF.4Znqe0.dYJRN.igjf""#;
    let cddl_input = r#"mypcre = tstr .pcre regexoptions
    
    regexoptions = "^[A-Z]$" / "[A-Za-z0-9]+@[A-Za-z0-9]+(\\.[A-Za-z0-9]+)+""#;

    validate_json_from_str(cddl_input, json_input)
  }

  #[test]
  fn validate_lt_control() -> Result {
    let json_input = r#"10.5"#;
    let cddl_input = r#"ltrule = float .lt 15.5"#;

    validate_json_from_str(cddl_input, json_input)
  }
}
