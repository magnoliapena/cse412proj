use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct ClassIdentifier {
    pub class_id: String, 
}
