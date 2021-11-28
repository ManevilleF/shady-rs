use crate::error::ShadyError;
use crate::node_operation::*;
use crate::{
    generate_unique_id, Connection, ConnectionMessage, ConnectionResponse, Input, InputField,
    NativeType, Output, OutputFields,
};
use serde::{Deserialize, Serialize};

/// A Shader node, representing an operation and input/output data
/// A Node also has a name, a unique id
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    name: String,
    id: String,
    input: Input,
    output: Output,
    operation: InternalNodeOperation,
}

impl Node {
    /// Instantiates a shader node with the given `name` and `operation`
    pub fn new(name: &str, operation: NodeOperation) -> Self {
        Self {
            name: name.to_string(),
            id: generate_unique_id(),
            input: operation.input(),
            output: operation.output(),
            operation: operation.into(),
        }
    }

    /// Instantiates a shader node with the given `name` and `operation` and with a custom unique id
    /// as `custom_id`
    pub fn new_with_custom_id(name: &str, custom_id: &str, operation: NodeOperation) -> Self {
        Self {
            name: name.to_string(),
            id: custom_id.to_string(),
            input: operation.input(),
            output: operation.output(),
            operation: operation.into(),
        }
    }

    /// Retrieves the node name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Retrieves the node unique id
    pub fn unique_id(&self) -> &String {
        &self.id
    }

    /// Retrieves the name and unique id of the Node formatted together
    pub fn unique_name(&self) -> String {
        format!("{}_{}", self.name, self.id)
    }

    fn find_input_field_pos(&self, field: &str) -> Result<usize, ShadyError> {
        let field_pos = self
            .input
            .fields
            .iter()
            .position(|(key, _f)| key == field)
            .ok_or_else(|| ShadyError::WrongFieldKey(field.to_string()))?;
        Ok(field_pos)
    }

    fn find_output_field_pos(&self, field: &str) -> Result<usize, ShadyError> {
        let field_pos = match self.output.fields() {
            OutputFields::SingleOutput(_) => return Err(ShadyError::SingleOutput(self.id.clone())),
            OutputFields::Fields(f) => f,
        }
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
        let (_k, f) = self.input.fields.get(pos)?;
        Some(f.glsl_type())
    }

    /// Retrieves an output field of the Shader Node
    pub fn get_output_field(&self, field: &str) -> Result<NativeType, ShadyError> {
        let pos = self.find_output_field_pos(field)?;
        let fields = self.output.fields().field_names();
        let (_k, f) = fields.get(pos).unwrap();
        Ok(*f)
    }

    /// Retrieves all input fields
    pub fn input_fields(&self) -> Vec<(String, InputField)> {
        self.input.fields.clone()
    }

    /// Retrieves all output fields
    pub fn output_fields(&self) -> OutputFields {
        self.output.fields()
    }

    /// Retrieves all the connections to other shader nodes
    pub fn node_connections(&self) -> Vec<String> {
        self.input
            .fields
            .iter()
            .filter_map(|(_, f)| match f.connection.as_ref()? {
                Connection::InputProperty { .. } | Connection::Constant { .. } => None,
                Connection::ComplexOutputNode { id, .. } | Connection::SingleOutputNode { id } => {
                    Some(id.clone())
                }
            })
            .collect()
    }

    /// Retrieves all the connections to other shader nodes
    pub fn connections(&self) -> Vec<(&String, &Connection)> {
        self.input
            .fields
            .iter()
            .filter_map(|(k, f)| f.connection.as_ref().map(|c| (k, c)))
            .collect()
    }

    /// Retrieves the optional `struct` declaration for the shader code
    pub fn struct_declaration(&self) -> Option<String> {
        self.output.custom_declaration()
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
        if let Connection::ComplexOutputNode { id, .. } | Connection::SingleOutputNode { id } =
            &connect_message.connection
        {
            if id == &self.id {
                return Err(ShadyError::SameNodeConnection(id.clone()));
            }
        }
        let field_pos = self.find_input_field_pos(target_field)?;
        let (_key, field) = self
            .input
            .fields
            .get_mut(field_pos)
            .ok_or_else(|| ShadyError::WrongFieldKey(target_field.to_string()))?;
        let expected_types = if field.tolerant {
            field.glsl_type.tolerated_input_types().to_vec()
        } else {
            vec![field.glsl_type()]
        };
        if !expected_types.contains(&connect_message.native_type) {
            return Err(ShadyError::WrongNativeType {
                input_type: connect_message.native_type,
                expected_types,
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
            .input
            .fields
            .get_mut(field_pos)
            .ok_or_else(|| ShadyError::WrongFieldKey(field_name.to_string()))?;
        Ok(field.connection.take())
    }

    /// Produces the associated shader code
    pub fn to_glsl(&self) -> String {
        format!(
            "{} {} = {}; // {} Node",
            self.output.glsl_type(),
            self.id,
            self.operation.to_glsl(self.input_field_glsl_values()),
            self.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_operation::NativeOperation;
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
                connection: Connection::ComplexOutputNode {
                    id: "some_var".to_string(),
                    field_name: "a".to_string(),
                },
                native_type: ScalarNativeType::Float.into(),
            },
        )
        .unwrap();
        node.connect_input(
            "y",
            ConnectionMessage {
                connection: Connection::ComplexOutputNode {
                    id: "other_var".to_string(),
                    field_name: "z".to_string(),
                },
                native_type: ScalarNativeType::Float.into(),
            },
        )
        .unwrap();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "vec2 {} = vec2(some_var.a, other_var.z); // test Node",
                node.id
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
            format!("vec2 {} = vec2(0.0, 0.0); // test Node", node.id)
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
            format!("vec3 {} = vec3(0.0, 0.0, 0.0); // test Node", node.id)
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
            format!("vec4 {} = vec4(0.0, 0.0, 0.0, 0.0); // test Node", node.id)
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
            format!("float {} = false ? 0.0 : 0.0; // test Node", node.id)
        );
    }
}
