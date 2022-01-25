//--------------------------------------------------------------------------
// usr dependencies
//--------------------------------------------------------------------------
use actix_web::{web, HttpResponse, ResponseError};
use thiserror::Error;
use log::{ info};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

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
#[derive(Serialize, Deserialize)]pub struct UsernameParam {username: String,}
#[derive(Serialize, Deserialize)]pub struct PasswdParam {passwd: String,}

//--------------------------------------------------------------------------
//  action_baseの読み込み
//--------------------------------------------------------------------------
use crate::controller::action_base;

/**
 *  引数での構造体宣言
 * //※GETの場合    <ハンドラ名>_struct:web::Query<<ハンドラ名>Param>
 * //※POSTの場合    <ハンドラ名>_struct:web::Form<<ハンドラ名>Param>
 *  最後はHashMapが作成されてそれを使用するので命名規則がカオスでもよい。
 * 手動で書くのは大変なので、ツールで生成するようにする
 */
pub async fn execute(
    username_struct: web::Query<UsernameParam>,
    passwd_struct: web::Form<PasswdParam>
) -> Result<HttpResponse, MyError> {
//--------------------------------------------------------------------------
// execute 処理開始
//--------------------------------------------------------------------------

    //--------------------------------------------------------------------------
    // 関数に渡せないので各種ハンドラのコピーを定義
    //  ※命名規則: let <ハンドラ名>_copy=<ハンドラ名>_struct.<ハンドラ名>
    //--------------------------------------------------------------------------
    let username_copy=&username_struct.username;//ハンドラ
    let passwd_copy=&passwd_struct.passwd;//ハンドラ

    //--------------------------------------------------------------------------
    // input_params 宣言 (HashMap形式  添え字はハンドラ名,<InputParametars構造体>)
    //--------------------------------------------------------------------------
    let mut input_params = HashMap::new();

    //--------------------------------------------------------------------------
    // input_result 宣言 (HashMap形式   詳細未定)
    //--------------------------------------------------------------------------
    let mut input_result  = HashMap::new();
    // 戻り値は 整数で管理 0→正常 5→バリバック 9→サーバエラー扱い
    input_result.insert(String::from("Result"), 0);

    //--------------------------------------------------------------------------
    // valliback_detail 宣言 (HashMap形式   詳細未定)
    //--------------------------------------------------------------------------
    let mut valliback_detail  = HashMap::new();

    //--------------------------------------------------------------------------
    // ハンドラをチェック関数を使って挿入する
    //--------------------------------------------------------------------------
    input_params.insert(String::from(r"username"),  action_base::InputParametars::set_input_parametars(
        username_copy.to_string(),                           //ハンドラ文字列(ハンドラ名はハッシュの添え字で判別)
        false,                                                   //文字列で扱うなら true それ以外なら false
        r"".to_string() ,                                 //ヴァリテーションバック時のメッセージ文字列
        -1,                                                     //最小値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        -1,                                                    //最大値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        r"*".to_string(),                               //正規表現チェック。境界値もこれで行う。全スルーは * 
    ));
    input_params.insert(String::from(r"passwd"),  action_base::InputParametars::set_input_parametars(
        passwd_copy.to_string(),                           //ハンドラ文字列(ハンドラ名はハッシュの添え字で判別)
        false,                                                   //文字列で扱うなら true それ以外なら false
        r"".to_string() ,                                 //ヴァリテーションバック時のメッセージ文字列
        0,                                                     //最小値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        4,                                                    //最大値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        r"*".to_string(),                               //正規表現チェック。境界値もこれで行う。全スルーは * 
    ));

    //--------------------------------------------------------------------------
    // 入力チェック結果を集計
    //--------------------------------------------------------------------------
    for (key, value) in &input_params {
        if value.result == false {
            //詳細を追加
            valliback_detail.insert(key, &value.result_msg);
            //全体の戻り値を更新
            input_result.insert(String::from("Result"), 5);
         }
        
    }
    //--------------------------------------------------------------------------
    // ヴァリテーションがある場合は input_resultを組み立てて返却
    //--------------------------------------------------------------------------
    let check_name = String::from("Result");
    let &check_result = input_result.get(&check_name ).unwrap();
    
    // ヴァリテーションの判定
    if  check_result ==5 {
        //--------------------------------------------------------------------------
        //該当する場合は ヴァリテーションバック処理 (応答コード200)
        //--------------------------------------------------------------------------
        return Ok(HttpResponse::InternalServerError().finish()) ;//能動的なエラー返却 500
    }

    info!("-完了- valliback_detail: {:?}",check_result); // スコープを抜けても使える。

    let response_body = "◇ 動作確認完了\n";
    // Ok(HttpResponse::InternalServerError().finish()) //能動的なエラー返却 500
    //Ok(HttpResponse::Unauthorized().finish())//能動的なエラー返却 401
    Ok(HttpResponse::Ok().body(response_body))
}




