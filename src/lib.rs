use regex::Regex;
use swc_core::{
  ecma::{
    ast::{ImportDecl, Program},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
  },
  plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
  fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
    let src = decl.src.value.to_string();

    let ts_re = Regex::new(r"^([\./].+)(\.ts)$").unwrap();
    let no_extension_re = Regex::new(r"^[\./].+[^(\.js)]$").unwrap();

    let ts_to_js = ts_re.replace(src.as_str(), "$1.js").to_string();
    decl.src = Box::new(
      no_extension_re
        .replace(ts_to_js.as_str(), "$0.js")
        .to_string()
        .into(),
    );
  }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
  program.fold_with(&mut as_folder(TransformVisitor))
}

#[cfg(test)]
mod transform_tests {
  use super::*;

  test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    add_extension_to_no_extension_import,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge";
    import HogeHoge from "./hogehoge";
    import { pppoe } from "../pppoe";
    "#,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    "#
  );

  test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    rewrite_extension_ts_to_js,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.ts";
    import HogeHoge from "./hogehoge.ts";
    import { pppoe } from "../pppoe.ts";
    "#,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    "#
  );

  test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    do_nothing_if_extension_is_js,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    "#,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    "#
  );

  test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    do_nothing_if_module_import,
    r#"
    import { Hoge, Fuga, Piyo } from "hogehoge";
    import HogeHoge from "hogehoge/hogehoge";
    import FugaFuga from "@hogehoge/fugafuga";
    "#,
    r#"
    import { Hoge, Fuga, Piyo } from "hogehoge";
    import HogeHoge from "hogehoge/hogehoge";
    import FugaFuga from "@hogehoge/fugafuga";
    "#
  );
}
