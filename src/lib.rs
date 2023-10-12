use globset::Glob;
use regex::Regex;
use serde::Deserialize;
use swc_core::{
  ecma::{
    ast::{ExportAll, ImportDecl, NamedExport, Program},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
  },
  plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
  aliases: Option<Vec<String>>,
}

pub struct TransformVisitor {
  aliases: Option<Vec<String>>,
}

impl TransformVisitor {
  pub fn new() -> Self {
    TransformVisitor { aliases: None }
  }

  pub fn set_config(&mut self, aliases: Option<Vec<String>>) {
    self.aliases = aliases;
  }
}

impl VisitMut for TransformVisitor {
  fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
    let src = decl.src.value.to_string();
    let alias_globs: Vec<Glob> = self
      .aliases
      .as_mut()
      .unwrap_or(&mut vec![])
      .iter()
      .map(|alias| Glob::new(alias).unwrap())
      .collect();

    decl.src = Box::new(transform_extension(src, alias_globs).into());
  }

  fn visit_mut_export_all(&mut self, decl: &mut ExportAll) {
    let src = decl.src.value.to_string();
    let alias_globs: Vec<Glob> = self
      .aliases
      .as_mut()
      .unwrap_or(&mut vec![])
      .iter()
      .map(|alias| Glob::new(alias).unwrap())
      .collect();

    decl.src = Box::new(transform_extension(src, alias_globs).into());
  }

  fn visit_mut_named_export(&mut self, named_export: &mut NamedExport) {
    let src = named_export
      .src
      .as_mut()
      .unwrap_or(&mut Box::new("".into()))
      .value
      .to_string();
    let alias_globs: Vec<Glob> = self
      .aliases
      .as_mut()
      .unwrap_or(&mut vec![])
      .iter()
      .map(|alias| Glob::new(alias).unwrap())
      .collect();

    named_export.src = Some(Box::new(transform_extension(src, alias_globs).into()));
  }
}

fn transform_extension(src: String, alias_glob: Vec<Glob>) -> String {
  let ts_re = Regex::new(r"^([\./].+)(\.ts)$").unwrap();

  let ts_to_js = ts_re.replace(src.as_str(), "$1.js").to_string();
  let no_extension_to_js = if ts_to_js.starts_with(".") && !ts_to_js.ends_with(".js") {
    format!("{}.js", ts_to_js)
  } else {
    ts_to_js
  };
  let new_src = alias_glob
    .iter()
    .any(|alias| {
      alias
        .compile_matcher()
        .is_match(no_extension_to_js.as_str())
    })
    .then(|| {
      let ts_re = Regex::new(r"^(.+)(\.ts)$").unwrap();

      let ts_to_js = ts_re
        .replace(no_extension_to_js.as_str(), "$1.js")
        .to_string();
      let no_extension_to_js = if !ts_to_js.ends_with(".js") {
        format!("{}.js", ts_to_js)
      } else {
        ts_to_js
      };

      no_extension_to_js
    })
    .unwrap_or(no_extension_to_js)
    .into();

  new_src
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
  let config = serde_json::from_str::<Config>(
    &metadata
      .get_transform_plugin_config()
      .expect("failed to get plugin config"),
  )
  .expect("invalid plugin config");

  let mut visitor = TransformVisitor::new();
  visitor.set_config(config.aliases);

  program.fold_with(&mut as_folder(visitor))
}

#[cfg(test)]
mod transform_tests {
  use swc_core::ecma::visit::Fold;

  use super::*;

  fn test_visitor() -> impl 'static + Fold + VisitMut {
    let visitor = TransformVisitor::new();

    as_folder(visitor)
  }

  fn test_visitor_with_alias() -> impl 'static + Fold + VisitMut {
    let mut visitor = TransformVisitor::new();
    visitor.set_config(Some(vec!["@/*".to_string()]));

    as_folder(visitor)
  }

  test!(
    Default::default(),
    |_| test_visitor(),
    add_extension_to_no_extension_import,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge";
    import HogeHoge from "./hogehoge";
    import { pppoe } from "../pppoe";
    import { utils } from "./utils";
    "#,
    r#"
    import { Hoge, Fuga, Piyo } from "./hogehoge.js";
    import HogeHoge from "./hogehoge.js";
    import { pppoe } from "../pppoe.js";
    import { utils } from "./utils.js";
    "#
  );

  test!(
    Default::default(),
    |_| test_visitor(),
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
    |_| test_visitor(),
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
    |_| test_visitor(),
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

  test!(
    Default::default(),
    |_| test_visitor_with_alias(),
    add_extension_to_no_extension_import_with_alias,
    r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge";
    import HogeHoge from "@/hogehoge";
    import { pppoe } from "@/pppoe";
    "#,
    r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge.js";
    import HogeHoge from "@/hogehoge.js";
    import { pppoe } from "@/pppoe.js";
    "#
  );

  test!(
    Default::default(),
    |_| test_visitor_with_alias(),
    rewrite_extension_ts_to_js_with_alias,
    r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge.ts";
    import HogeHoge from "@/hogehoge.ts";
    import { pppoe } from "@/pppoe.ts";
    "#,
    r#"
    import { Hoge, Fuga, Piyo } from "@/hogehoge.js";
    import HogeHoge from "@/hogehoge.js";
    import { pppoe } from "@/pppoe.js";
    "#
  );

  test!(
    Default::default(),
    |_| test_visitor_with_alias(),
    do_nothing_if_module_import_with_alias,
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

  test!(
    Default::default(),
    |_| test_visitor_with_alias(),
    for_export,
    r#"
    export { Hoge, Fuga, Piyo } from "hogehoge";
    export { pppoe } from "@/pppoe.ts";
    export { HogeHoge } from "@/hogehoge";
    "#,
    r#"
    export { Hoge, Fuga, Piyo } from "hogehoge";
    export { pppoe } from "@/pppoe.js";
    export { HogeHoge } from "@/hogehoge.js";
    "#
  );
}
