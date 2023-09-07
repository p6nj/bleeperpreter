use super::*;
use serde_json::from_str;

#[test]
fn string() {
    assert_eq!(
        Signal(Expr::from_str("sin(2*pi*f*t)").unwrap()),
        from_str(r#""sin(2*pi*f*t)""#).unwrap()
    );
}

#[test]
fn array() {
    assert_eq!(
        Signal(Expr::from_str("sin(2*pi*f*t)").unwrap()),
        from_str(r#"["sin(2*pi*f*t)"]"#).unwrap(),
        "single element"
    );
    assert_eq!(
        Signal(Expr::from_str("sin(2*pi*f*t)").unwrap()),
        from_str(r#"["a = 2", "sin(a*pi*f*t)"]"#).unwrap(),
        "1 variable"
    );
    assert_eq!(
        Signal(Expr::from_str("sin(2*pi*f*t)").unwrap()),
        from_str(r#"["f(x) = x", "sin(f(2)*pi*f*t)"]"#).unwrap(),
        "1 function"
    );
    assert_eq!(
        Signal(Expr::from_str("sin(2*pi*f*t)").unwrap()),
        from_str(r#"["a= 2", "f(x) = a", "sin(f(2)*pi*f*t)"]"#).unwrap(),
        "1 variable, 1 function"
    );
    assert_eq!(
        Signal(Expr::from_str("sin(2*pi*f*t)").unwrap()),
        from_str(r#"["e(x)=45*x", "f(x) = 2+e(0)", "sin(f(2)*pi*f*t)"]"#).unwrap(),
        "2 functions"
    );
}
