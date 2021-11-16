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
        let mut buffer = String::new();
        for id in self.ordered_nodes.iter() {
            buffer = format!(
                "{}{}\n    ",
                buffer,
                self.node_data.get(id).unwrap().glsl_code.clone()
            );
        }
        buffer
    }
}

impl Shader {
    fn get_property_declarations(&self) -> String {
        let mut property_declarations = String::new();
        let mut input: Vec<(String, String)> = self
            .input_properties
            .iter()
            .map(|(k, v)| (k.clone(), v.glsl_declaration()))
            .collect();
        input.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        for (_, value) in input {
            property_declarations = format!("{}{}\n", property_declarations, value);
        }
        let mut output: Vec<(String, String)> = self
            .output_properties
            .iter()
            .map(|(k, v)| (k.clone(), v.glsl_declaration()))
            .collect();
        output.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        for (_, value) in output {
            property_declarations = format!("{}{}\n", property_declarations, value);
        }
        property_declarations
    }

    fn output_property_generation(&self) -> String {
        let mut res = String::new();
        let mut output: Vec<(String, String)> = self
            .output_properties
            .iter()
            .map(|(k, v)| (k.clone(), v.to_glsl()))
            .collect();
        output.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        for (_, value) in output {
            res = format!("{}{}\n    ", res, value);
        }
        res
    }

    fn nodes_generation(&self, node_ids: Vec<String>) -> Result<NodeGeneration, ShadyError> {
        let mut res = NodeGeneration::default();
        let mut nodes_to_handle = node_ids;
        let mut required_nodes = Vec::new();

        for depth in 0..=self.max_processing_depth {
            log::debug!(
                "Depth: {}, {} nodes to declare",
                depth,
                nodes_to_handle.len()
            );
            nodes_to_handle.sort_unstable();
            nodes_to_handle.dedup();
            let mut tmp_nodes = Vec::new();
            for node_id in nodes_to_handle.iter() {
                log::trace!("Processing node {}", node_id);
                let node = self.get_node(node_id)?;
                let mut connections = node.node_connections();
                if !res.node_data.contains_key(node_id) {
                    res.node_data.insert(
                        node_id.clone(),
                        NodeData {
                            glsl_code: node.to_glsl(),
                            required_nodes: connections.clone(),
                        },
                    );
                }
                tmp_nodes.append(&mut connections);
            }
            // TODO: Check if this is enough to detect loops
            if required_nodes.contains(&nodes_to_handle) {
                return Err(ShadyError::NodeLoopDetected(nodes_to_handle));
            }
            required_nodes.push(nodes_to_handle);
            if tmp_nodes.is_empty() {
                log::info!("Finished processing nodes at depth {}", depth);
                break;
            } else if depth == self.max_processing_depth {
                return Err(ShadyError::MaxDepthReached(self.max_processing_depth));
            }
            nodes_to_handle = tmp_nodes;
        }
        required_nodes.reverse();
        for required_node in required_nodes {
            for node_id in required_node {
                if !res.ordered_nodes.contains(&node_id) {
                    res.ordered_nodes.push(node_id);
                }
            }
        }
        log::trace!("Node Generation: {:#?}", res);
        Ok(res)
    }

    pub fn to_glsl(&self) -> Result<String, ShadyError> {
        let property_declarations = self.get_property_declarations();

        let mut nodes_to_handle = Vec::new();
        // Output properties code
        for property in self.output_properties.values() {
            if let Some(Connection::NodeConnection { node_id, .. }) = &property.connection {
                nodes_to_handle.push(node_id.clone());
            }
        }
        let output_properties = self.output_property_generation();
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
            // Properties
            {properties}
            // Struct Declarations
            {structs}
            // Function declarations
            {functions}
            // Main Function
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
    use crate::node::{ConnectionAttempt, ConnectionTo, Input, InputField, Node, Output};
    use crate::shader::{InputProperty, OutputProperty};
    use crate::GlslType;

    fn init_basic_shader() -> Shader {
        let mut shader = Shader::new("Basic Shader".to_string());

        shader.add_input_property(InputProperty {
            name: "Gl_Position".to_string(),
            reference: "Gl_Pos123".to_string(),
            glsl_type: GlslType::Vec3,
            uniform: false,
        });
        shader.add_output_property(OutputProperty {
            name: "Out_Pos".to_string(),
            reference: "Out_Pos456".to_string(),
            glsl_type: GlslType::Vec3,
            connection: None,
        });
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "Gl_Pos123".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "Out_Pos456".to_string(),
                },
            })
            .unwrap();
        shader.save_to("test/basic_shader.yaml").unwrap();
        shader
    }

    fn init_simple_shader() -> Shader {
        let mut shader = Shader::new("Simple Shader".to_string());

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
        shader.save_to("test/simple_shader.yaml").unwrap();
        shader
    }

    fn init_example_shader_1() -> Shader {
        let mut shader = Shader::new("Shader Example 1".to_string());

        shader.add_input_property(InputProperty {
            name: "I".to_string(),
            reference: "i".to_string(),
            glsl_type: GlslType::Float,
            uniform: false,
        });
        shader.add_output_property(OutputProperty {
            name: "O_1".to_string(),
            reference: "o_1".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        shader.add_output_property(OutputProperty {
            name: "O_2".to_string(),
            reference: "o_2".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        shader.add_output_property(OutputProperty {
            name: "O_3".to_string(),
            reference: "o_3".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        let node_template = Node {
            name: "MyNode".to_string(),
            uuid: "node_azerty".to_string(),
            input_param: Input {
                fields: vec![
                    ("x".to_string(), InputField::new(GlslType::Float)),
                    ("y".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            output_param: Output::GlslType {
                glsl_type: GlslType::Float,
                field_name: "v".to_string(),
            },
            glsl_function: "my_func".to_string(),
        };
        shader.create_node(Node {
            name: "A".to_string(),
            uuid: "a".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "B".to_string(),
            uuid: "b".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "C".to_string(),
            uuid: "c".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "D".to_string(),
            uuid: "d".to_string(),
            ..node_template.clone()
        });
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "i".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "a".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "i".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "b".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "b".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "c".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "b".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "c".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "b".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "d".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "c".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "d".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_1".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "c".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_2".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "d".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_3".to_string(),
                },
            })
            .unwrap();
        shader.save_to("test/shader_example_1.yaml").unwrap();
        shader
    }

    fn init_example_shader_2() -> Shader {
        let mut shader = Shader::new("Shader Example 2".to_string());

        shader.add_input_property(InputProperty {
            name: "I_1".to_string(),
            reference: "i1".to_string(),
            glsl_type: GlslType::Float,
            uniform: false,
        });
        shader.add_input_property(InputProperty {
            name: "I_2".to_string(),
            reference: "i2".to_string(),
            glsl_type: GlslType::Float,
            uniform: false,
        });
        shader.add_output_property(OutputProperty {
            name: "O_1".to_string(),
            reference: "o_1".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        shader.add_output_property(OutputProperty {
            name: "O_2".to_string(),
            reference: "o_2".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        shader.add_output_property(OutputProperty {
            name: "O_3".to_string(),
            reference: "o_3".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        let node_template = Node {
            name: "MyNode".to_string(),
            uuid: "node_azerty".to_string(),
            input_param: Input {
                fields: vec![
                    ("x".to_string(), InputField::new(GlslType::Float)),
                    ("y".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            output_param: Output::GlslType {
                glsl_type: GlslType::Float,
                field_name: "v".to_string(),
            },
            glsl_function: "my_func".to_string(),
        };
        shader.create_node(Node {
            name: "A".to_string(),
            uuid: "a".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "B".to_string(),
            uuid: "b".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "C".to_string(),
            uuid: "c".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "D".to_string(),
            uuid: "d".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "E".to_string(),
            uuid: "e".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "F".to_string(),
            uuid: "f".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "G".to_string(),
            uuid: "g".to_string(),
            ..node_template.clone()
        });
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "i1".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "a".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "i1".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "e".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "i2".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "g".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "f".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_1".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "f".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "e".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "e".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "d".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "g".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "d".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "e".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "b".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "d".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "b".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "b".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "c".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "d".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "c".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "c".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_3".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "b".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_2".to_string(),
                },
            })
            .unwrap();
        shader.save_to("test/shader_example_2.yaml").unwrap();
        shader
    }

    fn init_looping_shader_1() -> Shader {
        let mut shader = Shader::new("Looping Shader 1".to_string());

        shader.add_input_property(InputProperty {
            name: "I".to_string(),
            reference: "i".to_string(),
            glsl_type: GlslType::Float,
            uniform: false,
        });
        shader.add_output_property(OutputProperty {
            name: "O_1".to_string(),
            reference: "o_1".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        shader.add_output_property(OutputProperty {
            name: "O_2".to_string(),
            reference: "o_2".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        shader.add_output_property(OutputProperty {
            name: "O_3".to_string(),
            reference: "o_3".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        let node_template = Node {
            name: "MyNode".to_string(),
            uuid: "node_azerty".to_string(),
            input_param: Input {
                fields: vec![
                    ("x".to_string(), InputField::new(GlslType::Float)),
                    ("y".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            output_param: Output::GlslType {
                glsl_type: GlslType::Float,
                field_name: "v".to_string(),
            },
            glsl_function: "my_func".to_string(),
        };
        shader.create_node(Node {
            name: "A".to_string(),
            uuid: "a".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "B".to_string(),
            uuid: "b".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "C".to_string(),
            uuid: "c".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "D".to_string(),
            uuid: "d".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "E".to_string(),
            uuid: "e".to_string(),
            ..node_template.clone()
        });
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "i".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "a".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "b".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "c".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "b".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "c".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "b".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "d".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "c".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "d".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "c".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "e".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "e".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "b".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_1".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "c".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_2".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "d".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o_3".to_string(),
                },
            })
            .unwrap();
        shader.save_to("test/looping_shader_1.yaml").unwrap();
        shader
    }

    fn init_looping_shader_2() -> Shader {
        let mut shader = Shader::new("Looping Shader 2".to_string());

        shader.add_input_property(InputProperty {
            name: "I".to_string(),
            reference: "i".to_string(),
            glsl_type: GlslType::Float,
            uniform: false,
        });
        shader.add_output_property(OutputProperty {
            name: "O".to_string(),
            reference: "o".to_string(),
            glsl_type: GlslType::Float,
            connection: None,
        });
        let node_template = Node {
            name: "MyNode".to_string(),
            uuid: "node_azerty".to_string(),
            input_param: Input {
                fields: vec![
                    ("x".to_string(), InputField::new(GlslType::Float)),
                    ("y".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            output_param: Output::GlslType {
                glsl_type: GlslType::Float,
                field_name: "v".to_string(),
            },
            glsl_function: "my_func".to_string(),
        };
        shader.create_node(Node {
            name: "A".to_string(),
            uuid: "a".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "B".to_string(),
            uuid: "b".to_string(),
            ..node_template.clone()
        });
        shader.create_node(Node {
            name: "C".to_string(),
            uuid: "c".to_string(),
            ..node_template.clone()
        });
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::PropertyConnection {
                    property_id: "i".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "a".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "b".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "b".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "c".to_string(),
                    field: "x".to_string(),
                },
            })
            .unwrap();
        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "c".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::ToNode {
                    id: "a".to_string(),
                    field: "y".to_string(),
                },
            })
            .unwrap();

        shader
            .connect(ConnectionAttempt {
                connection_from: Connection::NodeConnection {
                    node_id: "a".to_string(),
                    field_name: "v".to_string(),
                },
                connection_to: ConnectionTo::OutputProperty {
                    id: "o".to_string(),
                },
            })
            .unwrap();
        shader.save_to("test/looping_shader_2.yaml").unwrap();
        shader
    }

    mod declarations {
        use super::*;

        #[test]
        fn works_with_simple_shader() {
            let shader = init_simple_shader();

            assert_eq!(
                shader.get_property_declarations().as_str(),
                "in vec3 Gl_Pos123; // Gl_Position\n\
                out vec2 Out_Pos456; // Out_Pos\n"
            );
        }

        #[test]
        fn works_with_basic_shader() {
            let shader = init_basic_shader();

            assert_eq!(
                shader.get_property_declarations().as_str(),
                "in vec3 Gl_Pos123; // Gl_Position\n\
                out vec3 Out_Pos456; // Out_Pos\n"
            );
        }

        #[test]
        fn works_with_example_shader_1() {
            let shader = init_example_shader_1();

            assert_eq!(
                shader.get_property_declarations(),
                formatdoc! {"
                    in float i; // I
                    out float o_1; // O_1
                    out float o_2; // O_2
                    out float o_3; // O_3
                "}
            );
        }

        #[test]
        fn works_with_example_shader_2() {
            let shader = init_example_shader_2();

            assert_eq!(
                shader.get_property_declarations(),
                formatdoc! {"
                    in float i1; // I_1
                    in float i2; // I_2
                    out float o_1; // O_1
                    out float o_2; // O_2
                    out float o_3; // O_3
                "}
            );
        }
    }

    mod node_generation {
        use super::*;
    }

    mod glsl {
        use super::*;

        #[test]
        fn works_with_empty_shader() {
            let shader = Shader::default();
            assert_eq!(
                shader.to_glsl().unwrap().trim(),
                formatdoc! {"
                // Properties

                // Struct Declarations

                // Function declarations

                // Main Function
                void main() {{
                    
                    // Output properties
                    
                }}"}
                .as_str()
            )
        }

        #[test]
        fn works_with_basic_shader() {
            let shader = init_basic_shader();
            assert_eq!(
                shader.to_glsl().unwrap().trim(),
                formatdoc! {"
                // Properties
                in vec3 Gl_Pos123; // Gl_Position
                out vec3 Out_Pos456; // Out_Pos

                // Struct Declarations

                // Function declarations

                // Main Function
                void main() {{
                    
                    // Output properties
                    Out_Pos456 = Gl_Pos123; // Out_Pos
                    
                }}"}
                .as_str()
            )
        }

        #[test]
        fn works_with_simple_shader() {
            let shader = init_simple_shader();
            assert_eq!(
                shader.to_glsl().unwrap(),
                formatdoc! {"
                // Properties
                in vec3 Gl_Pos123; // Gl_Position
                out vec2 Out_Pos456; // Out_Pos

                // Struct Declarations

                // Function declarations

                // Main Function
                void main() {{
                    vec2 node_azerty = my_func(Gl_Pos123); // MyNode Node
                    
                    // Output properties
                    Out_Pos456 = node_azerty.out; // Out_Pos
                    
                }}
                "}
            )
        }

        #[test]
        fn works_with_complex_shader() {
            let shader = init_example_shader_1();
            assert_eq!(
                shader.to_glsl().unwrap(),
                formatdoc! {"
                // Properties
                in float i; // I
                out float o_1; // O_1
                out float o_2; // O_2
                out float o_3; // O_3

                // Struct Declarations

                // Function declarations

                // Main Function
                void main() {{
                    float a = my_func(i, 0.0); // A Node
                    float b = my_func(i, a.v); // B Node
                    float c = my_func(a.v, b.v); // C Node
                    float d = my_func(b.v, c.v); // D Node
                    
                    // Output properties
                    o_1 = a.v; // O_1
                    o_2 = c.v; // O_2
                    o_3 = d.v; // O_3
                    
                }}
               "}
            )
        }

        #[test]
        fn works_with_example_shader_2() {
            let shader = init_example_shader_2();
            assert_eq!(
                shader.to_glsl().unwrap(),
                formatdoc! {"
                // Properties
                in float i1; // I_1
                in float i2; // I_2
                out float o_1; // O_1
                out float o_2; // O_2
                out float o_3; // O_3

                // Struct Declarations

                // Function declarations

                // Main Function
                void main() {{
                    float a = my_func(i1, 0.0); // A Node
                    float f = my_func(a.v, 0.0); // F Node
                    float e = my_func(i1, f.v); // E Node
                    float g = my_func(i2, 0.0); // G Node
                    float d = my_func(e.v, g.v); // D Node
                    float b = my_func(e.v, d.v); // B Node
                    float c = my_func(b.v, d.v); // C Node
                    
                    // Output properties
                    o_1 = a.v; // O_1
                    o_2 = b.v; // O_2
                    o_3 = c.v; // O_3
                    
                }}
                "}
            )
        }

        #[should_panic = "NodeLoopDetected"]
        #[test]
        fn fails_with_looping_shader_1() {
            let shader = init_looping_shader_1();
            shader.to_glsl().unwrap();
        }

        #[should_panic = "NodeLoopDetected"]
        #[test]
        fn fails_with_looping_shader_2() {
            let shader = init_looping_shader_2();
            shader.to_glsl().unwrap();
        }
    }
}
