use captcha::{filters::Noise, Captcha};
use redis::{Commands, Client};
use salvo::handler;

use crate::GLOBAL_REDIS;

// 生成验证码
#[handler]
pub async fn get_captcha() -> (String, Option<String>) {
    // 生成验证码
    let mut captcha = Captcha::new();
    captcha
        .add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(220, 120);
    let (code, image) = (captcha.chars_as_string(), captcha.as_base64());
    // 保存验证码
    Client::set_ex::<&str, String, String>(&mut GLOBAL_REDIS.clone(), "123", code.clone(), 600).unwrap();
    // Client::set_ex(GLOBAL_REDIS.clone(), code.clone(), code.clone(), 60 * 5).unwrap();
    // 返回验证码
    (code, image)
}
