use salvo::prelude::ToSchema;
use serde::{Deserialize, Serialize};

use validator::Validate;

/// # CaptchaRes
/// 验证码返回数据模型
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct CaptchaRes {
    #[serde(rename = "captchaEnabled")]
    pub captcha_enabled: Option<bool>,
    pub img: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Validate, Clone, Debug, ToSchema)]
pub struct LoginReq {
    // #[validate(required(message = "验证码不能为空"))]
    #[validate(length(min = 1, message = "验证码不能为空"))]
    pub code: String,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: String,
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    #[validate(length(min = 1, message = "uuid不能为空"))]
    pub uuid: String,
}

// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginRes {
    pub token: String,
}
