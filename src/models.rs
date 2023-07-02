use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EditAttrForm {
    pub name: String,
    pub prompt: String,
    pub value: String,
}

impl From<web::Json<EditAttrForm>> for EditAttrForm {
    fn from(update_config: web::Json<EditAttrForm>) -> Self {
        EditAttrForm {
            name: update_config.name.clone(),
            prompt: update_config.prompt.clone(),
            value: update_config.value.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AttrNameText {
    pub name: String,
}
