#[macro_use]
extern crate neon;

use neon::vm::{Lock, JsResult, Module};
use neon::mem::Handle;
use neon::js::{JsString, JsFunction, JsObject, JsArray, Object};
use neon::js::class::{Class, JsClass};

struct Entry {
}

pub struct FSTree {
    entries: Vec<Entry>,
    size: usize
}

declare_types! {
    pub class JsFSTree for FSTree {
        init(call) {
            let scope = call.scope;
            let mut size = 0;

            if (call.arguments.len() > 0) {
                let options = try!(try!(call.arguments.require(scope, 0)).check::<JsObject>());
                let entries = try!(try!(options.get(scope, "entries")).check::<JsArray>());
                size = try!(entries.to_vec(scope)).len();

                println!("{}", try!(entries.to_vec(scope)).len());
            }

            Ok(FSTree {
                entries: vec![],
                size: size
            })
        }

        //method hello(call) {
            //let scope = call.scope;
            //let name = try!(try!(call.arguments.require(scope, 0)).check::<JsString>()).value();
            //let msg = call.arguments.this(scope).grab(|greeter| {
                //format!("{}, {}!", greeter.greeting, name)
            //});
            //Ok(JsString::new(scope, msg.as_str()).unwrap().upcast())
        //}
    }
}


register_module!(m, {
    let class: Handle<JsClass<JsFSTree>> = try!(JsFSTree::class(m.scope));
    let constructor: Handle<JsFunction<JsFSTree>> = try!(class.constructor(m.scope));
    try!(m.exports.set("FSTree", constructor));

    Ok(())
});
