//! wawk-hello — Example plugin for wawk.
//!
//! A minimal plugin that demonstrates the wawk plugin interface.
//! Use this as a starting point for building your own plugins.
//!
//! # Build
//! ```bash
//! cargo build --target wasm32-unknown-unknown --release
//! ```
//!
//! # Supported AWK Functions
//! | Function          | Description                      |
//! |-------------------|----------------------------------|
//! | `greet(name)`     | Returns "Hello, {name}!"         |
//! | `greet_lang(name)`| Returns greeting in one of 10 languages |

wit_bindgen::generate!({
    world: "wawk-plugin",
    path: "wit",
});

struct Component;

impl exports::wawk::plugins::external_functions::Guest for Component {
    fn call(name: String, args: Vec<String>) -> Option<String> {
        // Defensive bounds checks
        if name.len() > 64 {
            return Some("ERROR:function name too long".into());
        }
        if args.len() > 16 {
            return Some("ERROR:too many arguments".into());
        }

        match name.as_str() {
            "__init__" => Some("ok".into()),
            "__meta__" => Some(
                r#"{"name":"wawk-hello","version":"0.1.0","requires":[],"description":"Example plugin demonstrating the wawk plugin API","author":"Ailur Labs","homepage":"https://wawk.dev"}"#.into(),
            ),
            "greet" => {
                if args.is_empty() {
                    Some("ERROR:greet requires 1 argument, got 0".into())
                } else if args.len() > 1 {
                    Some(format!("ERROR:greet takes 1 argument, got {}", args.len()))
                } else {
                    Some(format!("Hello, {}!", &args[0]))
                }
            }
            "greet_lang" => {
                if args.is_empty() {
                    Some("ERROR:greet_lang requires 1 argument, got 0".into())
                } else if args.len() > 1 {
                    Some(format!("ERROR:greet_lang takes 1 argument, got {}", args.len()))
                } else {
                    let greetings = [
                        "Hello", "Hola", "Bonjour", "Ciao", "Konnichiwa",
                        "Ni hao", "Annyeonghaseyo", "Namaste", "Salam", "Zdravstvuyte",
                    ];
                    let idx = args[0].len() % greetings.len();
                    Some(format!("{}, {}!", greetings[idx], &args[0]))
                }
            }
            _ => None,
        }
    }
}

export!(Component);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exports::wawk::plugins::external_functions::Guest;

    #[test]
    fn greet_basic() {
        let result = Component::call("greet".into(), vec!["World".into()]);
        assert_eq!(result, Some("Hello, World!".into()));
    }

    #[test]
    fn greet_no_args() {
        let result = Component::call("greet".into(), vec![]);
        assert!(result.unwrap().starts_with("ERROR:"));
    }

    #[test]
    fn greet_too_many_args() {
        let result = Component::call("greet".into(), vec!["a".into(), "b".into()]);
        assert!(result.unwrap().starts_with("ERROR:"));
    }

    #[test]
    fn greet_lang_basic() {
        let result = Component::call("greet_lang".into(), vec!["World".into()]);
        let s = result.unwrap();
        assert!(s.ends_with(", World!"));
    }

    #[test]
    fn greet_lang_no_args() {
        let result = Component::call("greet_lang".into(), vec![]);
        assert!(result.unwrap().starts_with("ERROR:"));
    }

    #[test]
    fn greet_lang_too_many_args() {
        let result = Component::call("greet_lang".into(), vec!["a".into(), "b".into()]);
        assert!(result.unwrap().starts_with("ERROR:"));
    }

    #[test]
    fn dispatch_unknown_returns_none() {
        let result = Component::call("nonexistent".into(), vec!["arg".into()]);
        assert!(result.is_none());
    }

    // --- __init__ tests ---

    #[test]
    fn init_returns_ok() {
        let result = Component::call("__init__".into(), vec![]);
        assert_eq!(result, Some("ok".into()));
    }
    // --- __meta__ tests ---

    #[test]
    fn meta_returns_valid_json() {
        let result = Component::call("__meta__".into(), vec![]);
        let json_str = result.expect("__meta__ should return Some");
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json_str);
        assert!(parsed.is_ok(), "__meta__ should return valid JSON: {json_str}");
    }

    #[test]
    fn meta_contains_plugin_name() {
        let result = Component::call("__meta__".into(), vec![]);
        let json_str = result.expect("__meta__ should return Some");
        assert!(
            json_str.contains("wawk-hello"),
            "__meta__ should contain plugin name"
        );
    }

    #[test]
    fn meta_does_not_contain_license_required() {
        let result = Component::call("__meta__".into(), vec![]);
        let json_str = result.expect("__meta__ should return Some");
        assert!(
            !json_str.contains("license_required"),
            "__meta__ should NOT contain license_required (open source compliance)"
        );
    }

    // --- Defensive bounds check tests ---

    #[test]
    fn name_too_long_returns_error() {
        let long_name = "a".repeat(65);
        let result = Component::call(long_name, vec![]);
        assert!(result.unwrap().starts_with("ERROR:"));
    }

    #[test]
    fn too_many_args_returns_error() {
        let args: Vec<String> = (0..17).map(|i| format!("arg{i}")).collect();
        let result = Component::call("greet".into(), args);
        assert!(result.unwrap().starts_with("ERROR:"));
    }
}
