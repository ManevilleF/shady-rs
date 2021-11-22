pub use {connection::*, input::*, operation::*, output::*};

use crate::error::ShadyError;
use crate::{generate_uuid, NativeType};
use serde::{Deserialize, Serialize};

mod connection;
mod input;
mod operation;
mod output;

/// A Shader node, representing an operation and input/output data
/// A Node also has a name, a unique id
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    name: String,
    uuid: String,
    input_param: Input,
    output_param: Output,
    operation: InternalNodeOperation,
}

impl Node {
    /// Instantiates a shader node with the given `name` and `operation`
    pub fn new(name: &str, operation: NodeOperation) -> Self {
        Self {
            name: name.to_string(),
            uuid: generate_uuid(),
            input_param: operation.input(),
            output_param: operation.output(),
            operation: operation.into(),
        }
    }

    /// Instantiates a shader node with the given `name` and `operation` and with a custom unique id
    /// as `custom_id`
    pub fn new_with_custom_id(name: &str, custom_id: &str, operation: NodeOperation) -> Self {
        Self {
            name: name.to_string(),
            uuid: custom_id.to_string(),
            input_param: operation.input(),
            output_param: operation.output(),
            operation: operation.into(),
        }
    }

    /// Retrieves the node name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Retrieves the node unique id
    pub fn unique_id(&self) -> &String {
        &self.uuid
    }

    /// Retrieves the name and unique id of the Node formatted together
    pub fn unique_name(&self) -> String {
        format!("{}_{}", self.name, self.uuid)
    }

    fn find_input_field_pos(&self, field: &str) -> Result<usize, ShadyError> {
        let field_pos = self
            .input_param
            .fields
            .iter()
            .position(|(key, _f)| key == field)
            .ok_or_else(|| ShadyError::WrongFieldKey(field.to_string()))?;
        Ok(field_pos)
    }

    fn find_output_field_pos(&self, field: &str) -> Result<usize, ShadyError> {
        let field_pos = self
            .output_param
            .fields()
            .iter()
            .position(|(key, _f)| key == field)
            .ok_or_else(|| ShadyError::WrongFieldKey(field.to_string()))?;
        Ok(field_pos)
    }

    fn input_field_glsl_values(&self) -> Vec<String> {
        let mut fields = Vec::new();
        for (key, field) in self.input_fields().iter() {
            let val = match &field.connection {
                Some(connection) => connection.glsl_call(),
                None => {
                    log::warn!(
                        "No connection set for Node {}::{}. Using default value",
                        self.unique_name(),
                        key
                    );
                    field.glsl_type.default_glsl_value().to_string()
                }
            };
            fields.push(val);
        }
        fields
    }

    /// Retrieves an input field of the Shader Node
    pub fn get_input_field(&self, field: &str) -> Option<NativeType> {
        let pos = self.find_input_field_pos(field).ok()?;
        let (_k, f) = self.input_param.fields.get(pos)?;
        Some(f.glsl_type())
    }

    /// Retrieves an output field of the Shader Node
    pub fn get_output_field(&self, field: &str) -> Option<NativeType> {
        let pos = self.find_output_field_pos(field).ok()?;
        let fields = self.output_param.fields();
        let (_k, f) = fields.get(pos)?;
        Some(*f)
    }

    /// Retrieves all input fields
    pub fn input_fields(&self) -> Vec<(String, InputField)> {
        self.input_param.fields.clone()
    }

    /// Retrieves all input fields as `NativeType`
    pub fn input_field_types(&self) -> Vec<(String, NativeType)> {
        self.input_param
            .fields
            .iter()
            .map(|(k, i)| (k.clone(), i.glsl_type))
            .collect()
    }

    /// Retrieves all output fields
    pub fn output_field_types(&self) -> Vec<(String, NativeType)> {
        self.output_param.fields()
    }

    /// Retrieves all the connections to other shader nodes
    pub fn node_connections(&self) -> Vec<String> {
        self.input_param
            .fields
            .iter()
            .filter_map(|(_, f)| match f.connection.as_ref()? {
                Connection::InputProperty { .. } => None,
                Connection::Node { node_id, .. } => Some(node_id.clone()),
            })
            .collect()
    }

    /// Retrieves the optional `struct` declaration for the shader code
    pub fn struct_declaration(&self) -> Option<String> {
        self.output_param.custom_declaration()
    }

    /// Retrieves the optional function declaration for the shader code
    pub fn function_declaration(&self) -> Result<Option<String>, ShadyError> {
        self.operation.function_declaration()
    }

    /// Connects an output field (from a node or a property) to an input field of this Node.
    pub fn connect_input(
        &mut self,
        target_field: &str,
        connect_message: ConnectionMessage,
    ) -> Result<ConnectionResponse, ShadyError> {
        // Same connection check
        if let Connection::Node { node_id, .. } = &connect_message.connection {
            if node_id == &self.uuid {
                return Err(ShadyError::SameNodeConnection(node_id.clone()));
            }
        }
        let field_pos = self.find_input_field_pos(target_field)?;
        let (_key, field) = self
            .input_param
            .fields
            .get_mut(field_pos)
            .ok_or_else(|| ShadyError::WrongFieldKey(target_field.to_string()))?;
        let expected_type = field.glsl_type();
        if connect_message.glsl_type != expected_type {
            return Err(ShadyError::WrongNativeType {
                input_type: connect_message.glsl_type,
                expected_type,
            });
        }
        Ok(field.connection.replace(connect_message.connection))
    }

    /// Removes connection data as stored in the input field with `field_name`.
    ///
    /// If found, the removed connection is returned
    pub fn disconnect_field(&mut self, field_name: &str) -> Result<Option<Connection>, ShadyError> {
        let field_pos = self.find_input_field_pos(field_name)?;
        let (_key, field) = self
            .input_param
            .fields
            .get_mut(field_pos)
            .ok_or_else(|| ShadyError::WrongFieldKey(field_name.to_string()))?;
        Ok(field.connection.take())
    }

    /// Produces the associated shader code
    pub fn to_glsl(&self) -> String {
        format!(
            "{} {} = {}; // {} Node",
            self.output_param.glsl_type(),
            self.uuid,
            self.operation.to_glsl(self.input_field_glsl_values()),
            self.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NonScalarNativeType, ScalarNativeType};

    #[test]
    fn custom_vec2_node() {
        let mut node = Node::new(
            "test",
            NodeOperation::TypeConstruction(NonScalarNativeType::Vec2),
        );
        node.connect_input(
            "x",
            ConnectionMessage {
                connection: Connection::Node {
                    node_id: "some_var".to_string(),
                    field_name: "a".to_string(),
                },
                glsl_type: ScalarNativeType::Float.into(),
            },
        )
        .unwrap();
        node.connect_input(
            "y",
            ConnectionMessage {
                connection: Connection::Node {
                    node_id: "other_var".to_string(),
                    field_name: "z".to_string(),
                },
                glsl_type: ScalarNativeType::Float.into(),
            },
        )
        .unwrap();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "vec2 {} = vec2(some_var.a, other_var.z); // test Node",
                node.uuid
            )
        );
    }

    #[test]
    fn default_vec2_node() {
        let node = Node::new(
            "test",
            NodeOperation::TypeConstruction(NonScalarNativeType::Vec2),
        );
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec2 {} = vec2(0.0, 0.0); // test Node", node.uuid)
        );
    }

    #[test]
    fn default_vec3_node() {
        let node = Node::new(
            "test",
            NodeOperation::TypeConstruction(NonScalarNativeType::Vec3),
        );
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec3 {} = vec3(0.0, 0.0, 0.0); // test Node", node.uuid)
        );
    }

    #[test]
    fn default_vec4_node() {
        let node = Node::new(
            "test",
            NodeOperation::TypeConstruction(NonScalarNativeType::Vec4),
        );
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "vec4 {} = vec4(0.0, 0.0, 0.0, 0.0); // test Node",
                node.uuid
            )
        );
    }

    #[test]
    fn default_float_selection_node() {
        let node = Node::new(
            "test",
            NodeOperation::NativeOperation(NativeOperation::Selection(NativeType::Float)),
        );
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("float {} = false ? 0.0 : 0.0; // test Node", node.uuid)
        );
    }
}
