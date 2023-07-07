/// 用户相关接口

use captcha::{filters::Noise, Captcha};
use redis::{Client, Commands};
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use uuid::Uuid;
use validator::Validate;

use crate::model::user_model::{LoginReq, LoginRes};
use crate::model::ResObj;
use crate::util::res::{res_json_err, Res};
use crate::{model::user_model::CaptchaRes, util::res::res_json_ok, GLOBAL_REDIS};

// 生成验证码
#[endpoint(
    responses(
        (status_code = 200,body=ResObj<CaptchaRes>,description ="获取验证码")
    )
)]
pub async fn get_captcha() -> Res<CaptchaRes> {
    // 生成验证码
    let mut captcha = Captcha::new();
    captcha
        .add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(220, 120);
    let (code, image) = (captcha.chars_as_string(), captcha.as_base64());
    // 如果 image 不为空，则说明生成成功
    if let Some(image) = image {
        // 保存验证码
        let uuid = Uuid::new_v4().to_string();
        Client::set_ex::<&str, String, String>(&mut GLOBAL_REDIS.clone(), &uuid, code, 600)
            .unwrap();
        // 返回验证码
        Ok(res_json_ok(Some(CaptchaRes {
            img: image,
            captcha_enabled: Some(true),
            uuid,
        })))
    } else {
        Err(res_json_err("验证码生成失败".to_string()))
    }
}

// 登录
#[endpoint(
    responses(
        (status_code = 200,body=ResObj<LoginRes>,description ="登录")
    )
)]
pub fn login(login_body: JsonBody<LoginReq>) -> Res<LoginRes> {
    match login_body.validate() {
        Ok(_) => {},
        Err(e) => {
            print!("validate error is {}",e);
            return Err(res_json_err(e.to_string()));
        }
    }
    Ok(res_json_ok(Some(LoginRes {
        token: "123".to_string(),
    })))
}
