use salvo::{prelude::ToSchema, writer::Json};

use crate::model::ResObj;

#[allow(dead_code)]
pub fn res_ok<T: ToSchema>(data: Option<T>) -> ResObj<T> {
    ResObj::ok(data)
}

pub fn res_json_ok<T: ToSchema>(data: Option<T>) -> Json<ResObj<T>> {
    Json(ResObj::ok(data))
}

#[allow(dead_code)]
pub fn res_err<T:ToSchema>(msg:String)->ResObj<T>{
  ResObj::err(msg)
}

pub fn res_json_err<T:ToSchema>(msg:String)->Json<ResObj<T>>{
  Json(ResObj::err(msg))
}

pub type Res<T> = Result<Json<ResObj<T>>,Json<ResObj<()>>>;