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
                    let path = Path::new(decl.src.value.as_str()).with_extension("js");
                    if let Some(transformed_path) = path.to_str() {
                        decl.src = Box::new(Str::from(transformed_path));
                    }
                }
            }
            ModuleDecl::ExportAll(decl) => {
                let path = decl.src.value.as_str();

                if need_transform(path) {
                    let path = Path::new(decl.src.value.as_str()).with_extension("js");
                    if let Some(transformed_path) = path.to_str() {
                        decl.src = Box::new(Str::from(transformed_path));
                    }
                }
            }
            ModuleDecl::ExportNamed(decl) => {
                if let Some(src) = &decl.src {
                    let path = src.value.as_str();

                    if need_transform(path) {
                        let path = Path::new(src.value.as_str()).with_extension("js");
                        if let Some(transformed_path) = path.to_str() {
                            decl.src = Some(Box::new(Str::from(transformed_path)));
                        }
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
