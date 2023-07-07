use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};

pub mod user_router;
/// # Router
/// 路由模块

pub fn init() -> Router {
    println!("router init");

    let router = Router::new()
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .push(user_router::init_no_token_router());

    let doc = OpenApi::new("Swagger Api", "0.0.1").merge_router(&router);

    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}
