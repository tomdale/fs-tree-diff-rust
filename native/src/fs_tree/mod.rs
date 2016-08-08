use neon::vm::{Lock, Call, JsResult};
use neon::js::{JsValue, JsNull, JsFunction, JsString, JsNumber, JsObject, JsArray, Object};
use neon::js::class::*;
use neon::js::error::{JsError, Kind};
use neon::mem::Handle;

mod entry;

use fs_tree::entry::Entry;

pub struct FSTree {
    entries: Vec<Entry>,
    size: f64
}

declare_types! {
    pub class JsFSTree for FSTree {
        init(call) {
            let mut size = 0;

            if call.arguments.len() > 0 {
                let first_arg = try!(call.arguments.require(call.scope, 0));

                if first_arg.is_a::<JsObject>() {
                    let options = try!(first_arg.check::<JsObject>());
                    let entries = try!(try!(options.get(call.scope, "paths")).check::<JsArray>());
                    size = try!(entries.to_vec(call.scope)).len();
                }
            }

            Ok(FSTree {
                entries: vec![],
                size: size as f64
            })
        }

        method get(call) {
            let scope = call.scope;

            let attr: String = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();

            match &attr[..] {
                "size" => {
                    let size = call.arguments.this(scope).grab(|tree| { tree.size });
                    Ok(JsNumber::new(scope, size as f64).upcast())
                },
                _ => JsError::throw(Kind::TypeError, "property does not exist")
            }
        }

        method calculatePatch(call) {
            let scope = call.scope;

            Ok(JsString::new(scope, "hello").unwrap().upcast())
        }
    }
}

pub fn from_paths(call: Call) -> JsResult<JsFSTree> {
    let scope = call.scope;

    let jsPaths = try!(try!(call.arguments.require(scope, 0)).check::<JsArray>());
    let paths = try!(jsPaths.to_vec(scope));
    let mut rawPaths: Vec<String> = Vec::with_capacity(paths.len() as usize);

    for p in &paths {
        let path = try!(p.check::<JsString>()).value();
        rawPaths.push(path);
    }

    for i in 1..rawPaths.len() {
        let previous = &rawPaths[i-1];
        let current = &rawPaths[i];

        if previous.lt(current) {
            continue;
        } else {
            return JsError::throw(Kind::Error, &format!("expected entries[{}]: `{}` to be < entries[{}]: `{}`, but \
was not. Ensure your input is sorted and has no duplicate paths", i-1, &previous, i, &current));
        }
    }

    let class: Handle<JsClass<JsFSTree>> = try!(JsFSTree::class(scope));
    let constructor: Handle<JsFunction<JsFSTree>> = try!(class.constructor(scope));

    let options = JsObject::new(scope);

    try!(options.set("paths", jsPaths));
    let mut args: Vec<Handle<JsValue>> = vec![];

    args.push(options.upcast());

    constructor.construct(scope, args)
}
