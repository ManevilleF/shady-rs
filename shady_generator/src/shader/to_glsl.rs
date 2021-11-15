use crate::error::ShadyError;
use crate::node::Connection;
use crate::shader::Shader;

const MAX_DEPTH: u8 = u8::MAX;

impl Shader {
    fn get_property_declarations(&self) -> String {
        let mut property_declarations = String::new();
        for property in self.input_properties.values() {
            property_declarations =
                format!("{}\n{}", property_declarations, property.glsl_declaration());
        }
        for property in self.output_properties.values() {
            property_declarations =
                format!("{}\n{}", property_declarations, property.glsl_declaration());
        }
        property_declarations
    }

    pub fn to_glsl(&self) -> Result<String, ShadyError> {
        let property_declarations = self.get_property_declarations();

        let mut struct_declarations = Vec::new();
        // TODO: implement the function loading
        let mut function_declarations = vec![""];
        let mut main_content = String::new();
        let mut nodes_to_handle = Vec::new();
        let mut handled_nodes = Vec::new();
        for property in self.output_properties.values() {
            main_content = format!("{}\n{}", main_content, property.to_glsl());
            if let Some(Connection::NodeConnection { node_id, .. }) = &property.connection {
                nodes_to_handle.push(node_id.clone());
            }
        }
        for depth in 0..=MAX_DEPTH {
            log::debug!(
                "Depth: {}, {} nodes to declare",
                depth,
                nodes_to_handle.len()
            );
            nodes_to_handle.dedup();
            let mut tmp_nodes = Vec::new();
            for node_id in nodes_to_handle.drain(..) {
                log::trace!("Processing node {}", node_id);
                if handled_nodes.contains(&node_id) {
                    return Err(ShadyError::NodeLoopDetected(node_id));
                }
                let node = self.get_node(&node_id)?;
                main_content = format!("{}\n{}", main_content, node.to_glsl());
                let mut connections = node.node_connections();
                tmp_nodes.append(&mut connections);
                if let Some(declaration) = node.struct_declaration() {
                    struct_declarations.push(declaration);
                }
                handled_nodes.push(node_id);
            }
            nodes_to_handle = tmp_nodes;
            if nodes_to_handle.is_empty() {
                log::info!("Finished processing nodes at depth {}", depth);
                break;
            } else if depth == MAX_DEPTH {
                return Err(ShadyError::MaxDepthReached(MAX_DEPTH));
            }
        }

        struct_declarations.dedup();
        function_declarations.dedup();
        let struct_declarations = struct_declarations.join("\n\n");
        let function_declarations = function_declarations.join("\n\n");

        Ok(formatdoc! {"
            {properties}
            
            {structs}

            {functions}

            void main() {{
                {main}
            }}
        ", 
            properties = property_declarations,
            structs = struct_declarations,
            functions = function_declarations,
            main = main_content
        })
    }
}
