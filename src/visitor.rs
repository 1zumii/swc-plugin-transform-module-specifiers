use crate::config::PluginConfig;
use std::path::Path;
use swc_core::ecma::{
    ast::{ModuleDecl, Str},
    visit::VisitMut,
};

#[derive(Default)]
pub struct TransformVisitor {
    pub config: PluginConfig,
}

impl TransformVisitor {
    fn need_transform(&self, path: &str) -> bool {
        if !(path.starts_with("./") || path.starts_with("../")) {
            return false;
        }
        match Path::new(path).extension() {
            Some(ext) => {
                let map = self.config.get_specifier_map();

                map.contains_key(ext.to_str().unwrap())
            }
            None => false,
        }
    }

    fn transform_path(&self, path: &str) -> String {
        let path = Path::new(path);
        let map = self.config.get_specifier_map();

        let source_ext = path.extension().unwrap().to_str().unwrap();
        let target_ext = map.get(source_ext).unwrap();

        path.with_extension(target_ext).display().to_string()
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module_decl(&mut self, decl: &mut ModuleDecl) {
        match decl {
            ModuleDecl::Import(decl) => {
                let path = decl.src.value.as_str();

                if self.need_transform(path) {
                    decl.src = Box::new(Str::from(self.transform_path(decl.src.value.as_str())));
                }
            }
            ModuleDecl::ExportAll(decl) => {
                let path = decl.src.value.as_str();

                if self.need_transform(path) {
                    decl.src = Box::new(Str::from(self.transform_path(decl.src.value.as_str())));
                }
            }
            ModuleDecl::ExportNamed(decl) => {
                if let Some(src) = &decl.src {
                    let path = src.value.as_str();

                    if self.need_transform(path) {
                        decl.src =
                            Some(Box::new(Str::from(self.transform_path(src.value.as_str()))));
                    }
                }
            }
            _ => {}
        }
    }
}
