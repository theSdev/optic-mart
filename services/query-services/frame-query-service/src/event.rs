use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrameCreatedData {
	pub brand_name: String,
	pub colors: Vec<String>,
	pub cover_image: Option<String>,
	pub description: Option<String>,
	pub has_case: bool,
	pub materials: Vec<String>,
	pub model_name: String,
	pub other_images: Vec<String>,
	pub owner_id: String,
	pub price: f32,
	pub privacy_mode: i16,
}
