use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hex::encode_hex;
use crate::msg::elem::flash_image::FlashImage;
use crate::pb::msg;
use crate::pb::msg::CustomFace;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GroupImage {
    pub image_id: String,
    pub file_id: i64,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub md5: Vec<u8>,
    pub orig_url: Option<String>,
    pub image_type: i32,
}

impl GroupImage {
    pub fn flash(self) -> FlashImage {
        FlashImage::from(self)
    }

    pub fn url(&self) -> String {
        if let Some(orig_url) = &self.orig_url {
            format!("https://gchat.qpic.cn{}", orig_url)
        } else {
            format!(
                "https://gchat.qpic.cn/gchatpic_new/0/0-0-{}/0?term=2",
                encode_hex(&self.md5).to_uppercase()
            )
        }
    }
}

impl From<GroupImage> for msg::CustomFace {
    fn from(e: GroupImage) -> Self {
        msg::CustomFace {
            file_type: Some(66),
            useful: Some(1),
            biz_type: Some(5),
            width: Some(e.width),
            height: Some(e.height),
            file_id: Some(e.file_id as i32),
            file_path: Some(e.image_id),
            md5: Some(e.md5),
            image_type: Some(e.image_type),
            size: Some(e.size),
            flag: Some(vec![0; 4]),
            ..Default::default()
        }
    }
}

impl From<GroupImage> for Vec<msg::elem::Elem> {
    fn from(e: GroupImage) -> Self {
        vec![{ msg::elem::Elem::CustomFace(e.into()) }]
    }
}

impl From<msg::CustomFace> for GroupImage {
    fn from(custom_face: CustomFace) -> Self {
        if custom_face.md5().is_empty() {
            return Self::default();
        }
        // guild image todo
        return Self {
            file_id: custom_face.file_id() as i64,
            image_id: custom_face.file_path().to_owned(),
            size: custom_face.size(),
            width: custom_face.width(),
            height: custom_face.height(),
            orig_url: custom_face.orig_url,
            md5: custom_face.md5.unwrap_or_default(),
            image_type: custom_face.image_type.unwrap_or(1000),
        };
    }
}

fn to_uuid(md5: &str) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        &md5[0..8],
        &md5[8..12],
        &md5[12..16],
        &md5[16..20],
        &md5[20..32],
    )
}

pub fn calculate_image_resource_id(md5: &[u8], no_dash: bool) -> String {
    let mut r = "{".to_owned();
    let md5 = crate::hex::encode_hex(md5).to_uppercase();
    if no_dash {
        r.push_str(&md5);
    } else {
        r.push_str(&to_uuid(&md5));
    }
    r.push_str("}.png");
    r
}

impl fmt::Display for GroupImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[GroupImage: {}]", self.url())
    }
}
