//--------------------------------------------------------------------------
// actix_web モジュール
//--------------------------------------------------------------------------
use actix_web::middleware::Logger;
use actix_web::{web, get ,App, HttpRequest, HttpResponse, HttpServer, ResponseError,Responder};
use thiserror::Error;
use log::{debug, error, info, trace, warn, LevelFilter,SetLoggerError};

//--------------------------------------------------------------------------
// ResponseError のラッパー宣言。独自のエラー処理に使用
//--------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum MyError {}
impl ResponseError for MyError {}

// MyError は actix_web::ResponseError を実装しているので、
// index の戻り値に MyError を使うことが出来ます。
pub async fn execute(web::Path(_user_id): web::Path<String>) -> Result<HttpResponse, MyError> {
    info!("user_id is {}", _user_id);
    let response_body = "Hello world!req_test";
    Ok(HttpResponse::Ok().body(response_body))
}

