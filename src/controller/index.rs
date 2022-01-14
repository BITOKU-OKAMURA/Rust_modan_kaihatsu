//--------------------------------------------------------------------------
// usr dependencies
//--------------------------------------------------------------------------
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use log::{
//    debug, 
//    error, 
    info, 
//    trace, 
//    warn, 
//    LevelFilter,
//    SetLoggerError
};

//--------------------------------------------------------------------------
// ResponseError のラッパー宣言。独自のエラー処理に使用
//--------------------------------------------------------------------------
#[derive(Error, Debug)]
pub enum MyError {}
impl ResponseError for MyError {}

// MyError は actix_web::ResponseError を実装しているので、
// index の戻り値に MyError を使うことが出来ます。
pub async fn execute() -> Result<HttpResponse, MyError> {
    info!("NISHI8888");
    let response_body = "西8888";
    Ok(HttpResponse::Ok().body(response_body))
}


