pub fn friendly_join(strings: Vec<&str>) -> String {
  match strings.len() {
    0 => String::new(),
    1 => String::from(strings[0]),
    2 => format!("{} and {}", strings[0], strings[1]),
    _ => {
      let mut s = String::new();
      for i in 0..strings.len() - 1 {
        s.push_str(strings[i]);
        s.push_str(", ");
      }
      s.push_str("and ");
      s.push_str(strings[strings.len() - 1]);
      s
    }
  }
}

#[test]
fn test_friendly_join() {
  assert_eq!(friendly_join(vec![]), "");
  assert_eq!(friendly_join(vec!["foo"]), "foo");
  assert_eq!(friendly_join(vec!["foo", "bar"]), "foo and bar");
  assert_eq!(friendly_join(vec!["foo", "bar", "baz"]), "foo, bar, and baz");
}
