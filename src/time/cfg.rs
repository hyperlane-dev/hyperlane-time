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
    println!("当前时间: {}", current_time());
    println!("当前日期: {}", current_date());
    println!("GMT日期: {}", current_date_gmt());
    println!("时间戳(毫秒): {}", current_timestamp_millis());
    println!("时间戳(微秒): {}", current_timestamp_micros());
}
