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

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginReq{
  pub code:Option<String>,
  pub password:Option<String>,
  pub username:Option<String>,
  pub uuid:Option<String>
}


// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginRes{
  pub token:String,
}
