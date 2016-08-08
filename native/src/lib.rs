#[macro_use]
extern crate neon;

mod fs_tree;

use neon::mem::Handle;
use neon::js::class::{Class, JsClass};
use neon::js::{JsFunction, Object};
use fs_tree::{JsFSTree, from_paths};

register_module!(m, {
  try!(m.export("fromPaths", from_paths));

  let class: Handle<JsClass<JsFSTree>> = try!(JsFSTree::class(m.scope));
  let constructor: Handle<JsFunction<JsFSTree>> = try!(class.constructor(m.scope));
  try!(m.exports.set("FSTree", constructor));

  Ok(())
});
