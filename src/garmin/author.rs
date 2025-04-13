use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    user_profile_pk: u32,
    display_name: String,
    full_name: String,
    profile_img_name_large: Option<String>,
    profile_img_name_medium: String,
    profile_img_name_small: String,
    user_pro: bool,
    vivokid_user: bool
}


impl Default for Author {
    fn default() -> Self {
        Author{
            user_profile_pk: 100441918,
            display_name: "ea022d71-d096-4261-a5cd-9fb3aa2bf953".to_string(),
            full_name: "Eirik Vårli Lid".to_string(),
            profile_img_name_large: None,
            profile_img_name_medium: "ff381b64-b337-4dd4-8b7e-3ca04c1b0528-100441918.png".to_string(),
            profile_img_name_small: "bce65d55-03d1-427f-a603-ed8e9a545985-100441918.png".to_string(),
            user_pro: false,
            vivokid_user: false,
        }
    }
}