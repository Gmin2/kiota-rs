#[derive(Debug, Clone, Default, PartialEq)]
pub struct Post {
    /// Stores additional data not described in the OpenAPI description found when deserializing. Can be used for serialization as well.
    #[serde(rename = "AdditionalData")]
    additional_data: HashMap<String, serde_json::Value>,
    /// The body property
    body: Option<String>,
    /// The id property
    id: Option<i32>,
    /// The title property
    title: Option<String>,
    /// The userId property
    #[serde(rename = "userId")]
    user_id: Option<i32>,
    /// Instantiates a new Post and sets the default values.
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_from_discriminator_value(_parse_node: &dyn ParseNode) -> Result<Self, KiotaError> {
        Ok(Self::default())
    }
    pub fn get_additional_data(&self) -> &HashMap<String, serde_json::Value> {
        &self.additional_data
    }
    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }
    /// The deserialization information for the current model
    pub fn get_field_deserializers(&self) -> FieldDeserializers {
        let mut map = FieldDeserializers::new();
        map.insert("body".to_string(), Box::new(|n: &dyn ParseNode| {
            // TODO: set field from n.get_string_value()
            Ok(())
        }));
        map.insert("id".to_string(), Box::new(|n: &dyn ParseNode| {
            // TODO: set field from n.get_string_value()
            Ok(())
        }));
        map.insert("title".to_string(), Box::new(|n: &dyn ParseNode| {
            // TODO: set field from n.get_string_value()
            Ok(())
        }));
        map.insert("userId".to_string(), Box::new(|n: &dyn ParseNode| {
            // TODO: set field from n.get_string_value()
            Ok(())
        }));
        map
    }
    pub fn get_id(&self) -> &Option<i32> {
        &self.id
    }
    pub fn get_title(&self) -> &Option<String> {
        &self.title
    }
    pub fn get_user_id(&self) -> &Option<i32> {
        &self.user_id
    }
    /// Serializes information the current object
    pub fn serialize(&self, writer: &mut dyn SerializationWriter) -> Result<(), KiotaError> {
        writer.write_object_value(Some("body"), &self.body as &dyn Parsable, &[])?;
        writer.write_object_value(Some("id"), &self.id as &dyn Parsable, &[])?;
        writer.write_object_value(Some("title"), &self.title as &dyn Parsable, &[])?;
        writer.write_object_value(Some("userId"), &self.user_id as &dyn Parsable, &[])?;
        writer.write_additional_data(&self.additional_data)?;
        Ok(())
    }
    pub fn set_additional_data(&mut self, value: ()) {
        self.additional_data = value;
    }
    pub fn set_body(&mut self, value: ()) {
        self.body = value;
    }
    pub fn set_id(&mut self, value: ()) {
        self.id = value;
    }
    pub fn set_title(&mut self, value: ()) {
        self.title = value;
    }
    pub fn set_user_id(&mut self, value: ()) {
        self.user_id = value;
    }
}
