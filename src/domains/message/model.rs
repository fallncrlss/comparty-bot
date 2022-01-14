#[derive(serde::Deserialize)]
pub struct CASResponse {
    pub ok: bool,
    description: String,
}
