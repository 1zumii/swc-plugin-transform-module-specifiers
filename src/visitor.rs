use swc_core::ecma::{ast::ImportDecl, visit::VisitMut};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_import_decl(&mut self, decl: &mut ImportDecl) {
        dbg!(&decl.src.value);
    }
}
