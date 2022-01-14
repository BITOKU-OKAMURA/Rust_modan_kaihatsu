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
//※POSTの場合 web::Form<構造体名>
//※GETの場合    web::Query<構造体名>
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
pub async fn execute(
    web::Path(_user_id): web::Path<String>,
    info_struct: web::Query<GetParams>,
    post_struct: web::Form<PostParams>
) -> Result<HttpResponse, MyError> {
    info!("user_id is {}", _user_id);
    info!("user_name is {}", info_struct.username);
    info!("◇ passwd is {}", post_struct.passwd);
    let response_body = "Hello world!req_test\n";
    // Ok(HttpResponse::InternalServerError().finish()) //能動的なエラー返却 500
    //Ok(HttpResponse::Unauthorized().finish())//能動的なエラー返却 401
    Ok(HttpResponse::Ok().body(response_body))
}






