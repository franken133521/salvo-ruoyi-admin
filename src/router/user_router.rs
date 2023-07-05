use salvo::Router;

use crate::controller::user_controller;

pub fn init_no_token_router() -> Router {
    let router = Router::new();
    router.push(
        // 验证码接口
        Router::with_path("captchaImage").get(user_controller::get_captcha)
    )
}