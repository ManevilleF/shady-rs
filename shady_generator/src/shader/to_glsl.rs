use crate::error::ShadyError;
use crate::node::Connection;
use crate::shader::Shader;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct NodeData {
    glsl_code: String,
    required_nodes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct NodeGeneration {
    pub node_data: HashMap<String, NodeData>,
    pub ordered_nodes: Vec<String>,
}

impl NodeGeneration {
    fn to_glsl(&self) -> String {
        let res: Vec<String> = self
            .ordered_nodes
            .iter()
            .map(|id| self.node_data.get(id).unwrap().glsl_code.clone())
            .collect();
        res.join("\n")
    }
}

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

    fn nodes_generation(&self, node_ids: Vec<String>) -> Result<NodeGeneration, ShadyError> {
        let mut res = NodeGeneration::default();
        let mut nodes_to_handle = node_ids;

        for depth in 0..=self.max_processing_depth {
            log::debug!(
                "Depth: {}, {} nodes to declare",
                depth,
                nodes_to_handle.len()
            );
            nodes_to_handle.dedup();
            let mut tmp_nodes = Vec::new();
            for node_id in nodes_to_handle.iter() {
                log::trace!("Processing node {}", node_id);
                if res.ordered_nodes.contains(node_id) {
                    return Err(ShadyError::NodeLoopDetected(node_id.clone()));
                }
                let node = self.get_node(node_id)?;
                let mut connections = node.node_connections();
                res.node_data.insert(
                    node_id.clone(),
                    NodeData {
                        glsl_code: node.to_glsl(),
                        required_nodes: connections.clone(),
                    },
                );
                tmp_nodes.append(&mut connections);
            }
            res.ordered_nodes.append(&mut nodes_to_handle);
            nodes_to_handle = tmp_nodes;
            if nodes_to_handle.is_empty() {
                log::info!("Finished processing nodes at depth {}", depth);
                break;
            } else if depth == self.max_processing_depth {
                return Err(ShadyError::MaxDepthReached(self.max_processing_depth));
            }
        }
        Ok(res)
    }

    pub fn to_glsl(&self) -> Result<String, ShadyError> {
        let property_declarations = self.get_property_declarations();

        let mut output_properties = String::new();
        let mut nodes_to_handle = Vec::new();
        // Output properties code
        for property in self.output_properties.values() {
            output_properties = format!("{}{}\n", output_properties, property.to_glsl());
            if let Some(Connection::NodeConnection { node_id, .. }) = &property.connection {
                nodes_to_handle.push(node_id.clone());
            }
        }
        let main_content = self.nodes_generation(nodes_to_handle)?;

        // TODO: implement the struct declarations
        let mut struct_declarations = vec![""];
        // TODO: implement the function loading
        let mut function_declarations = vec![""];

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

                // Output properties
                {output}
            }}
        ", 
            properties = property_declarations,
            structs = struct_declarations,
            functions = function_declarations,
            main = main_content.to_glsl(),
            output = output_properties
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glsl::GlslType;
    use crate::node::{ConnectionAttempt, ConnectionTo, Input, InputField, Node, Output};
    use crate::shader::{InputProperty, OutputProperty};

    fn init_simple_shader() -> Shader {
        let mut shader = Shader::new("TestShader".to_string());

        shader.add_input_property(InputProperty {
            name: "Gl_Position".to_string(),
            reference: "Gl_Pos123".to_string(),
            glsl_type: GlslType::Vec3,
            uniform: false,
        });
        shader.add_output_property(OutputProperty {
            name: "Out_Pos".to_string(),
            reference: "Out_Pos456".to_string(),
            glsl_type: GlslType::Vec2,
            connection: None,
        });
        shader.create_node(Node {
            name: "MyNode".to_string(),
            uuid: "node_azerty".to_string(),
            input_param: Input {
                fields: vec![("pos".to_string(), InputField::new(GlslType::Vec3))],
            },
            output_param: Output::GlslType {
                glsl_type: GlslType::Vec2,
                field_name: "out".to_string(),
            },
            glsl_function: "my_func".to_string(),
        });
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "Gl_Pos123".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "node_azerty".to_string(),
                    field: "pos".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "node_azerty".to_string(),
                    field_name: "out".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "Out_Pos456".to_string(),
                },
            })
            .unwrap();
        shader
    }

    mod declarations {
        use super::*;

        #[test]
        fn works_with_simple_shader() {
            let shader = init_simple_shader();

            assert_eq!(
                shader.get_property_declarations().as_str(),
                "\nin vec3 Gl_Pos123; // Gl_Position\n\
                out vec2 Out_Pos456; // Out_Pos"
            );
        }
    }

    mod node_generation {
        use super::*;
    }

    mod glsl {
        use super::*;

        #[test]
        fn works_with_simple_shader() {
            let shader = init_simple_shader();
            assert_eq!(
                shader.to_glsl().unwrap().trim(),
                formatdoc! {"
                in vec3 Gl_Pos123; // Gl_Position
                out vec2 Out_Pos456; // Out_Pos





                void main() {{
                    vec2 node_azerty = my_func(Gl_Pos123);

                    // Output properties
                    Out_Pos456 = node_azerty.out; // Out_Pos

                }}"}
                .as_str()
            )
        }

        #[test]
        fn works_with_empty_shader() {
            let shader = Shader::default();
            assert_eq!(
                shader.to_glsl().unwrap().trim(),
                formatdoc! {"
                void main() {{
                    

                    // Output properties
                    
                }}"}
                .as_str()
            )
        }
    }
}
