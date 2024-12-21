// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditorData {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "imageURL")]
    pub image_url: String,
    #[serde(rename = "link")]
    pub link: String,
}

impl AuditorData {
    pub fn new(name: String, image_url: String, link: String) -> AuditorData {
        AuditorData {
            name,
            image_url,
            link,
        }
    }
}
