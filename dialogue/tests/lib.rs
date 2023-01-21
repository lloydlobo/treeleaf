// #[macro_use]
// mod test;

fn path(s: &str) -> String {
    if cfg!(windows) {
        s.replace('/', "\\")
    } else {
        s.into()
    }
}

fn path_for_regex(s: &str) -> String {
    if cfg!(windows) {
        s.replace('/', "\\\\")
    } else {
        s.into()
    }
}

use pretty_assertions::assert_eq;

macro_rules! test {
  {
    name: $name:ident,
    $(justfile: $justfile:expr,)?
    $(args: ($($arg:tt),*),)?
    $(env: { $($env_key:literal : $env_value:literal,)* },)?
    $(stdin: $stdin:expr,)?
    $(stdout: $stdout:expr,)?
    $(stdout_regex: $stdout_regex:expr,)?
    $(stderr: $stderr:expr,)?
    $(stderr_regex: $stderr_regex:expr,)?
    $(status: $status:expr,)?
    $(shell: $shell:expr,)?
  } => {
    #[test]
    fn $name() {
      let test = crate::test::Test::new();

      $($(let test = test.arg($arg);)*)?
      $($(let test = test.env($env_key, $env_value);)*)?
      $(let test = test.justfile($justfile);)?
      $(let test = test.shell($shell);)?
      $(let test = test.status($status);)?
      $(let test = test.stderr($stderr);)?
      $(let test = test.stderr_regex($stderr_regex);)?
      $(let test = test.stdin($stdin);)?
      $(let test = test.stdout($stdout);)?
      $(let test = test.stdout_regex($stdout_regex);)?

      test.run();
    }
  }
}

pub(crate) struct Output {
    pub(crate) stdout: String,
    // pub(crate) tempdir: TempDir,
}

pub(crate) struct Test {
    // pub(crate) args: Vec<String>,
    // pub(crate) current_dir: PathBuf,
    // pub(crate) env: BTreeMap<String, String>,
    // pub(crate) justfile: Option<String>,
    // pub(crate) shell: bool,
    // pub(crate) status: i32,
    // pub(crate) stderr: String,
    // pub(crate) stderr_regex: Option<Regex>,
    // pub(crate) stdin: String,
    // pub(crate) stdout: String,
    // pub(crate) stdout_regex: Option<Regex>,
    // // pub(crate) tempdir: TempDir,
    // pub(crate) test_round_trip: bool,
    // pub(crate) unindent_stdout: bool,
}

pub(crate) fn tempdir() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("just-test-tempdir")
        .tempdir()
        .expect("failed to create temporary directory")
}

// #[test]
// fn test_tempdir_is_set() {
//     Test::new()
//         .justfile(
//             "
//       set tempdir := '.'
//       foo:
//           #!/usr/bin/env bash
//           cat just*/foo
//       ",
//         )
//         .shell(false)
//         .tree(tree! {
//           foo: {
//           }
//         })
//         .current_dir("foo")
//         .stdout(if cfg!(windows) {
//             "
//       cat just*/foo
//       "
//         } else {
//             "
//       #!/usr/bin/env bash
//       cat just*/foo
//       "
//         })
//         .run();
// }
