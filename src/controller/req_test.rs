//--------------------------------------------------------------------------
// usr dependencies
//--------------------------------------------------------------------------
use actix_web::middleware::Logger;
use actix_web::{web, get ,App, HttpRequest, HttpResponse, HttpServer, ResponseError,Responder};
use thiserror::Error;
use log::{debug, error, info, trace, warn, LevelFilter,SetLoggerError};
use serde::Deserialize;
use serde::Serialize;

//--------------------------------------------------------------------------
// ResponseError のラッパー宣言。独自のエラー処理に使用
//--------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum MyError {}
impl ResponseError for MyError {}

//--------------------------------------------------------------------------
// 画面遷移別個別対応
//--------------------------------------------------------------------------
//web::Form<PostParams>
//web::Query<GetParams>
#[derive(Serialize, Deserialize)]
pub struct GetParams {
    username: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostParams {
    passwd: String,
}

// MyError は actix_web::ResponseError を実装しているので、
// index の戻り値に MyError を使うことが出来ます。
pub async fn execute(web::Path(_user_id): web::Path<String>,info_struct: web::Query<GetParams>,post_struct: web::Form<PostParams>) -> Result<HttpResponse, MyError> {
    info!("user_id is {}", _user_id);
    info!("user_name is {}", info_struct.username);
    info!("◇ passwd is {}", post_struct.passwd);
    let response_body = "Hello world!req_test";
    Ok(HttpResponse::Ok().body(response_body))
}




