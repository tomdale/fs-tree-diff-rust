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

impl FSTree {
    fn calculatePatch(&self, theirs: &Vec<Entry>) {
        let ours = &self.entries;

        let operations: Vec<(&str, String, Entry)> = vec![];

        let mut i = 0;
        let mut j = 0;

        let mut removals = vec![];

        while i < ours.len() && j < theirs.len() {
            let x = &ours[i];
            let y = &theirs[j];

            if x.relative_path.lt(&y.relative_path) {
                i += 1;

                if x.is_directory {
                    removals.push(add_command(&x));
                }
            } else {
                i += 1;
                j += 1;
            }
        }

        println!("{:?}", operations);
    }
}

fn add_command(entry: &Entry) -> (&'static str, String) {
    ("mkdir", entry.relative_path.clone())
}

declare_types! {
    pub class JsFSTree for FSTree {
        init(call) {
            let mut size = 0;
            let mut entries: Vec<Entry> = vec![];

            if call.arguments.len() > 0 {
                let first_arg = try!(call.arguments.require(call.scope, 0));

                if first_arg.is_a::<JsObject>() {
                    let options = try!(first_arg.check::<JsObject>());
                    let jsEntries = try!(try!(try!(options.get(call.scope, "paths")).check::<JsArray>()).to_vec(call.scope));
                    size = jsEntries.len();
                    entries = jsEntries.iter().map(|e| {
                        let path = match e.check::<JsString>() {
                            Ok(v) => v.value(),
                            Err(e) => "".to_string()
                        };

                        Entry::new(path)
                    }).collect();
                }
            }

            Ok(FSTree {
                entries: entries,
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

            let otherTree = try!(call.arguments.require(scope, 0));
            let theirEntries = try!(otherTree.check::<JsFSTree>()).grab(|tree| {
                tree.entries.clone()
            });

            call.arguments.this(scope).grab(|tree| {
                tree.calculatePatch(&theirEntries);
            });

                //let otherTree = try!(otherTree.check::<JsFSTree>());

                //otherTree.grab(|rawOtherTree| {
                    //Ok(tree.calculatePatch(rawOtherTree.entries))
                //})
            //});

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
