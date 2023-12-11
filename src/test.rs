use crate::visitor::TransformVisitor;
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
    as_folder(TransformVisitor)
}

// DEBUG:
const TEST_CONTENT: &str = r#"
    import { v1, v2, v3 } from './file1.ts';
    import type { t1, t2, t3 } from '../file2.ts';
    import fun1 from './file3.ts';
    import externFun from '@some/lib';

    fun1();

    function fun2() {
        const v4 = import('./file4.ts');
    }
"#;

test_inline!(SYNTAX, transformer, debug, TEST_CONTENT, TEST_CONTENT);
