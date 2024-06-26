use std::collections::HashMap;

use crate::{config::PluginConfig, visitor::TransformVisitor};
use swc_core::ecma::{
    parser::{Syntax, TsConfig},
    transforms::testing::{test, test_inline, Tester},
    visit::{as_folder, Fold},
};

const SYNTAX: Syntax = Syntax::Typescript(TsConfig {
    tsx: true,
    decorators: false,
    dts: false,
    no_early_errors: false,
    disallow_ambiguous_jsx_like: true,
});

fn transformer(_: &mut Tester) -> impl Fold {
    as_folder(TransformVisitor::default())
}

// TODO: control quote(single?) in transformation

test_inline!(
    SYNTAX,
    transformer,
    /* name */ ts2js,
    /* input */
    r#"
    import { v1, v2, v3 } from './file1.ts';
    import type { t1, t2, t3 } from '../file2.ts';
    import fun1 from './file3.ts';
    import externFun from '@some/lib';

    export * from './file5.ts';
    export { v5, type t4 } from '../file6.ts';
    "#,
    /* output */
    r#"
    import { v1, v2, v3 } from "./file1.js";
    import type { t1, t2, t3 } from "../file2.js";
    import fun1 from "./file3.js";
    import externFun from '@some/lib';

    export * from "./file5.js";
    export { v5, type t4 } from "../file6.js";
    "#
);

test_inline!(
    SYNTAX,
    transformer,
    /* name */ ts2js_import,
    /* input */
    r#"
    import { v1, v2, v3 } from './file1.ts';
    "#,
    /* output */
    r#"
    import { v1, v2, v3 } from "./file1.js";
    "#
);

test_inline!(
    SYNTAX,
    |_| as_folder(TransformVisitor {
        config: PluginConfig::new(HashMap::from([
            (String::from("ts"), String::from("js")),
            (String::from("mts"), String::from("mjs"))
        ]))
    }),
    /* name */ default_cjs_with_esm_import,
    /* input */
    r#"
    import { v1, v2, v3 } from './file1.ts';
    import { v4, v5, v6 } from './file2.mts';
    "#,
    /* output */
    r#"
    import { v1, v2, v3 } from "./file1.js";
    import { v4, v5, v6 } from "./file2.mjs";
    "#
);
