use salvo::prelude::ToSchema;
use serde::{Deserialize, Serialize};

/// # CaptchaRes
/// 验证码返回数据模型
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct CaptchaRes {
    #[serde(rename = "captchaEnabled")]
    pub captcha_enabled: Option<bool>,
    pub img: String,
    pub uuid: String,
}
