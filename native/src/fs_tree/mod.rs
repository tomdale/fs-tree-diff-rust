use neon::scope::Scope;
use neon::vm::{Lock, Call, FunctionCall, JsResult};
use neon::js::{JsValue, JsNull, JsFunction, JsString, JsNumber, JsInteger, JsObject, JsArray, Object};
use neon::js::class::{JsClass, Class};
use neon::js::error::{JsError, Kind};
use neon::mem::Handle;

pub mod entry;

use fs_tree::entry::{Entry, JsEntry};

#[derive(Clone)]
pub struct FSTree {
    entries: Vec<Entry>,
    size: f64
}

#[derive(Debug)]
struct Command<'a>(&'static str, String, &'a Entry);

impl FSTree {
    fn calculatePatch<'a>(&'a self, theirs: &'a Vec<Entry>) -> Vec<Command> {
        let ours = &self.entries;

        let mut operations: Vec<Command> = vec![];
        let mut removals: Vec<Command> = vec![];

        let mut i = 0;
        let mut j = 0;

        while i < ours.len() && j < theirs.len() {
            let x = &ours[i];
            let y = &theirs[j];

            if x.relative_path.lt(&y.relative_path) {
                i += 1;

                let command = remove_command(x);

                if x.is_directory {
                    removals.push(command);
                } else {
                    operations.push(command);
                }
            } else if x.relative_path.gt(&y.relative_path) {
                j += 1;
                operations.push(add_command(&y))
            } else {
                if !is_equal(&x, &y) {
                    let command = update_command(&y);

                    if x.is_directory {
                        removals.push(command)
                    } else {
                        operations.push(command)
                    }
                }

                i += 1;
                j += 1;
            }
        }

        while i < ours.len() {
            removals.push(add_command(&ours[i]));
            i += 1;
        }

        while j < theirs.len() {
            operations.push(add_command(&theirs[j]));
            j += 1;
        }

        removals.reverse();
        operations.append(&mut removals);
        operations
    }
}

fn is_equal(entryA: &Entry, entryB: &Entry) -> bool {
    if entryA.is_directory && entryB.is_directory {
        return true
    }

    entryA.relative_path == entryB.relative_path
}

fn add_command(entry: &Entry) -> Command {
    let command = if entry.is_directory { "mkdir" } else { "create" };
    Command(command, entry.relative_path.clone(), entry)
}

fn remove_command(entry: &Entry) -> Command {
    let command = if entry.is_directory  { "rmdir" } else { "unlink" };
    Command(command, entry.relative_path.clone(), entry)
}

fn update_command(entry: &Entry) -> Command {
    Command("change", entry.relative_path.clone(), entry)
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

            let mut this = call.arguments.this(scope);

            let commands: Vec<Command> = this.grab(|tree| {
                tree.calculatePatch(&theirEntries)
            });

            to_js_array(scope, commands)
        }
    }
}

fn to_js_array<'a, S: Scope<'a>>(scope: &mut S, commands: Vec<Command>) -> JsResult<'a, JsValue> {
    let array: Handle<JsArray> = JsArray::new(scope, commands.len() as u32);

    for i in 0..commands.len() {
        try!(array.set((i as u32), try!(command_to_js_array(scope, &commands[i]))));
    }

    Ok(array.upcast())
}

fn command_to_js_array<'a, S: Scope<'a>>(scope: &mut S, command: &Command) -> JsResult<'a, JsValue> {
    let array: Handle<JsArray> = JsArray::new(scope, 2);
    let path = &command.1[..];

    array.set(0, JsString::new(scope, command.0).unwrap());
    array.set(1, JsString::new(scope, path).unwrap());
    array.set(2, try!(entry_to_js_entry(scope, command.2)));

    Ok(array.upcast())
}

fn entry_to_js_entry<'a, S: Scope<'a>>(scope: &mut S, entry: &Entry) -> JsResult<'a, JsEntry> {
    let js_entry_class = JsEntry::class(scope);
    let class: Handle<JsClass<JsEntry>> = try!(js_entry_class);
    let constructor: Handle<JsFunction<JsEntry>> = try!(class.constructor(scope));

    let path = JsString::new(scope, &entry.relative_path).unwrap();

    let mut args: Vec<Handle<JsValue>> = vec![];

    args.push(path.upcast());

    constructor.construct(scope, args)
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
