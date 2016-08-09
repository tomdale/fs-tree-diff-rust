use neon::scope::Scope;
use neon::vm::{Lock, Call, FunctionCall, JsResult};
use neon::js::{JsValue, JsBoolean, JsNull, JsFunction, JsString, JsNumber, JsInteger, JsObject, JsArray, Object};
use neon::js::class::*;
use neon::js::error::{JsError, Kind};
use neon::mem::Handle;

#[derive(Clone, Debug)]
pub struct Entry {
  pub relative_path: String,
  pub is_directory: bool
}

impl Entry {
    pub fn new(path: String) -> Entry {
        let is_directory = path.chars().last().unwrap() == '/';

        Entry {
            relative_path: path,
            is_directory: is_directory
        }
    }
}

declare_types! {
    pub class JsEntry for Entry {
        init(call) {
            let scope = call.scope;
            let relative_path = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());

            Ok(Entry::new(relative_path.value()))
        }

        constructor(call) {
            let scope = call.scope;

            let mut this = call.arguments.this(scope);

            let (relative_path, is_directory) = this.grab(|entry| {
                (entry.relative_path.clone(), entry.is_directory)
            });

            let mode = if is_directory { 16877 } else { 0 };
            this.set("mode", JsInteger::new(scope, mode));
            this.set("mtime", JsInteger::new(scope, 0));
            this.set("size", JsInteger::new(scope, 0));
            this.set("relativePath", JsString::new(scope, &relative_path[..]).unwrap());

            Ok(None)
        }

        method isDirectory(call) {
            let scope = call.scope;
            let is_directory = call.arguments.this(scope).grab(|entry| {
                entry.is_directory
            });

            Ok(JsBoolean::new(scope, is_directory).upcast())
        }
    }
}
