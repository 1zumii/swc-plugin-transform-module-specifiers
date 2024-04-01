use std::path::Path;
use swc_core::ecma::{
    ast::{ModuleDecl, Str},
    visit::VisitMut,
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_module_decl(&mut self, decl: &mut ModuleDecl) {
        match decl {
            ModuleDecl::Import(decl) => {
                let path = decl.src.value.as_str();

                if need_transform(path) {
                    decl.src = Box::new(Str::from(transform_path(decl.src.value.as_str())));
                }
            }
            ModuleDecl::ExportAll(decl) => {
                let path = decl.src.value.as_str();

                if need_transform(path) {
                    decl.src = Box::new(Str::from(transform_path(decl.src.value.as_str())));
                }
            }
            ModuleDecl::ExportNamed(decl) => {
                if let Some(src) = &decl.src {
                    let path = src.value.as_str();

                    if need_transform(path) {
                        decl.src = Some(Box::new(Str::from(transform_path(src.value.as_str()))));
                    }
                }
            }
            _ => {}
        }
    }
}

fn need_transform(path: &str) -> bool {
    if !(path.starts_with("./") || path.starts_with("../")) {
        return false;
    }
    match Path::new(path).extension() {
        Some(ext) => ext == "ts",
        None => false,
    }
}

fn transform_path(path: &str) -> String {
    Path::new(path).with_extension("js").display().to_string()
}
