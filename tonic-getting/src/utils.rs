use std::time::SystemTime;

pub fn now_millis() -> i64 {
  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64
}
