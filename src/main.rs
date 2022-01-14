//--------------------------------------------------------------------------
// actix_web モジュール
//--------------------------------------------------------------------------
/*
use actix_web::middleware::Logger;
use actix_web::{Query,web, get ,App, HttpRequest, HttpResponse, HttpServer, ResponseError,Responder};
use thiserror::Error;
use log::{debug, error, info, trace, warn, LevelFilter,SetLoggerError};

//--------------------------------------------------------------------------
// ResponseError のラッパー宣言。独自のエラー処理に使用
//--------------------------------------------------------------------------
#[derive(Error, Debug)]
enum MyError {}
impl ResponseError for MyError {}
*/
use actix_web::{web, App,  HttpServer};
use actix_web::middleware::Logger;

//--------------------------------------------------------------------------
// 外部モジュールの読み込み ※クレートにする必要が薄いのでソースで管理
//--------------------------------------------------------------------------
mod resorce_module;//ログとかDBとかのモジュール
mod controller;//コントローラ

//curl -XPOST -d 'passwd=abcdefg12345678' http://192.168.186.13:8080/input_param/?username=aaa

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    resorce_module::logs::log4rs_init("actix_web=info","/tmp/foo.log");
    HttpServer::new(move || App::new().wrap(Logger::default())
        .route("/", web::get().to(controller::index::execute))
        //.route("/req_test/{_user_id}", web::get().to(controller::req_test::execute))
        .route("/req_test/{_user_id}", web::post().to(controller::req_test::execute))
        .route("/input_param/", web::post().to(controller::input_param::execute))
    )
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

