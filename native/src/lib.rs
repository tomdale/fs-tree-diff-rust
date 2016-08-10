#[macro_use]
extern crate neon;
extern crate itertools;

mod fs_tree;

use neon::mem::Handle;
use neon::js::class::{Class, JsClass};
use neon::js::{JsFunction, Object};
use fs_tree::{JsFSTree, from_paths, calculate_patch_from_paths};
use fs_tree::entry::JsEntry;

register_module!(m, {
  try!(m.export("fromPaths", from_paths));

  let class: Handle<JsClass<JsFSTree>> = try!(JsFSTree::class(m.scope));
  let constructor: Handle<JsFunction<JsFSTree>> = try!(class.constructor(m.scope));
  try!(m.exports.set("FSTree", constructor));

  let class: Handle<JsClass<JsEntry>> = try!(JsEntry::class(m.scope));
  let constructor: Handle<JsFunction<JsEntry>> = try!(class.constructor(m.scope));
  try!(m.exports.set("Entry", constructor));

  try!(m.export("calculatePatchFromPaths", calculate_patch_from_paths));

  Ok(())
});
