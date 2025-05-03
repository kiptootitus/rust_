pub mod helpersmod {
  pub fn get_full_name(first: &str, middle: &str, last: &str) -> String {
      format!("{} {} {}", first.trim(), middle.trim(), last.trim())
  }
}
