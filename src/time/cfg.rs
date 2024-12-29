#[test]
fn test_lang() {
    use crate::time::r#type::from_env_var;
    println!("test_lang: {}", from_env_var());
}

#[test]
fn test_now_time() {
    use crate::*;
    println!("test_now_time: {}", current_time());
}

#[test]
fn test_now_day() {
    use crate::*;
    println!("test_now_day: {}", current_date());
}
