use salvo::{Router, prelude::{Logger, CatchPanic}};

pub mod user_router;
/// # Router
/// 路由模块

pub fn init() -> Router {
    println!("router init");
    
    Router::new().hoop(Logger::new())
        .hoop(CatchPanic::new())
        .push(user_router::init_no_token_router())
}