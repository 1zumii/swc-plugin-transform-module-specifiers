mod config;
#[cfg(test)]
mod tests;
mod visitor;

use config::PluginConfig;
use swc_core::ecma::{
    ast::Program,
    visit::{as_folder, FoldWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use visitor::TransformVisitor;

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = PluginConfig::new(
        serde_json::from_str(
            &metadata
                .get_transform_plugin_config()
                .expect("failed to get plugin config for transform-module-specifiers"),
        )
        .expect("invalid packages"),
    );

    program.fold_with(&mut as_folder(TransformVisitor { config }))
}
