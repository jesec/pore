use std::collections::HashMap;

lazy_static! {
  static ref HOOKS: HashMap<String, &'static str> = {
    let mut map = HashMap::new();

    macro_rules! add_hook {
      ($hook:expr) => {
        // include_str! only takes string literals, so we have to repeat ourselves. (rust-lang/rust#53749)
        map.insert($hook.into(), include_str!($hook));
      }
    }

    add_hook!("commit-msg");
    add_hook!("pre-auto-gc");

    map
  };
}

pub fn hooks() -> &'static HashMap<String, &'static str> {
  &HOOKS
}
