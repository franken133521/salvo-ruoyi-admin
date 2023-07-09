mod controller;
mod model;
mod router;
mod util;

use fast_log::{
    consts::LogSize,
    plugin::{file_split::RollingType, packer::LogPacker},
    Config,
};
use log::LevelFilter;
use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_mysql::driver::MysqlDriver;
use redis::Client;
use salvo::{prelude::TcpListener, Listener, Server};

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

pub static GLOBAL_REDIS: Lazy<Client> = Lazy::new(|| {
    let client = redis::Client::open("redis://127.0.0.1/").expect("redis连接失败");
    client.get_connection().expect("redis连接失败");
    client
});

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt().init();
    // 初始化日志
    fast_log::init(
        Config::new()
            .level(LevelFilter::Debug)
            .console()
            .chan_len(Some(100000))
            .file_split(
                "target/logs/salvo-ruoyi-admin.log",
                LogSize::MB(1),
                RollingType::All,
                LogPacker {},
            ),
    )
    .unwrap();
    // 初始化数据库连接
    let mysql_uri = "mysql://root:123456@localhost/ry-vue";
    RB.link(MysqlDriver {}, mysql_uri).await.unwrap();
    // 初始化路由
    let router = router::init();
    // 启动服务
    let acceptor = TcpListener::new("0.0.0.0:8090").bind().await;
    Server::new(acceptor).serve(router).await;
}
