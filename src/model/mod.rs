use salvo::prelude::ToSchema;
use serde::Serialize;

pub mod user_model;

/// # Res
/// 返回数据模型
#[derive(Debug, Serialize, ToSchema)]
pub struct ResObj<T: ToSchema + 'static> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// # PageRes
/// 分页返回数据模型
#[derive(Debug, Serialize, ToSchema)]
pub struct PageRes<T: ToSchema + 'static> {
    pub code: i32,
    pub msg: String,
    pub rows: Vec<T>,
    pub total: u64,
}

impl<T: ToSchema> ResObj<T> {
    pub fn ok(data: Option<T>) -> Self {
        Self {
            code: 0,
            msg: "访问成功".to_string(),
            data,
        }
    }
    #[allow(dead_code)]
    pub fn custom_code(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }

    pub fn err(err: String) -> Self {
        Self {
            code: 500,
            msg: err,
            data: None,
        }
    }
}
