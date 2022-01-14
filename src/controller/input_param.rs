//--------------------------------------------------------------------------
// usr dependencies
//--------------------------------------------------------------------------
use actix_web::{web, HttpResponse, ResponseError};
use thiserror::Error;
use log::{ info};
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
/**
 *  構造体の命名規則：<ハンドラ名>Param ※ハンドラ名はsnake case
 *  ！！ 型指定は画面遷移を避ける目的で、Stringで統一。
 * 
 */
#[derive(Serialize, Deserialize)]pub struct usernameParam {username: String,}
#[derive(Serialize, Deserialize)]pub struct passwdParam {passwd: String,}

/**
 *  引数での構造体宣言
 * //※GETの場合    <ハンドラ名>_struct:web::Query<<ハンドラ名>Param>
 * //※POSTの場合    <ハンドラ名>_struct:web::Form<<ハンドラ名>Param>
 *  最後はHashMapが作成されてそれを使用するので命名規則がカオスでもよい。
 * 手動で書くのは大変なので、ツールで生成するようにする
 */
pub async fn execute(
    username_struct: web::Query<usernameParam>,
    passwd_struct: web::Form<passwdParam>
) -> Result<HttpResponse, MyError> {
//--------------------------------------------------------------------------
// execute 処理開始
//--------------------------------------------------------------------------
    




    info!("user_name is {}", username_struct.username);
    info!("◇ passwd is {}", passwd_struct.passwd);
    let response_body = "◇ 動作確認完了\n";
    // Ok(HttpResponse::InternalServerError().finish()) //能動的なエラー返却 500
    //Ok(HttpResponse::Unauthorized().finish())//能動的なエラー返却 401
    Ok(HttpResponse::Ok().body(response_body))
}




