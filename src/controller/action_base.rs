//--------------------------------------------------------------------------
// Regix関連
//--------------------------------------------------------------------------
extern crate regex;
use regex::Regex;

//--------------------------------------------------------------------------
// 構造体:InputParametars
//--------------------------------------------------------------------------
#[derive(Debug)]
pub struct InputParametars {
    pub string_type: bool ,     // 文字列で扱うなら true それ以外なら false
    pub str_value:String,       // 文字列でのパンドラ値
    pub str_length:i32,               // 文字列長
    pub int_value:i32,          //整数でのパンドラ値
    pub float_value:f64,       // floatでのパンドラ値   
    pub result: bool,           // 値チェックの結果 OK なら true NG なら false
    pub result_msg:String,       // ValidationBack時のメッセージ
} 
 
//--------------------------------------------------------------------------
// トレイト (構造体:InputParametars)
//--------------------------------------------------------------------------
impl InputParametars {
    //---------------------------------------------------------------------------------
    // トレイト内関数:set_input_parametars
    // ***** 構造体InputParametars として値を代入する。引数項目以外は演算して代入を実施 ******
    // * 目的:フォームハンドラーの値を精査し、サーバエラーで無くヴァリテーションバックとして返却*
    // * 境界値チェックや不正アクセスの精査を体系的に実施                                                          *
    // ***************************************************************************
    //--------------------------------------------------------------------------------- 
    pub fn set_input_parametars (
        //----- 引数一覧 -----//
        args: String,                  //ハンドラ文字列(ハンドラ名はハッシュの添え字で判別)
        string_type_in: bool,   //文字列で扱うなら true それ以外なら false
        mut message_in: String,    //ヴァリテーションバック時のメッセージ文字列
        min_in:i32,                  //最小値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        max_in:i32,                 //最大値、文字列の場合は文字列数 -1で無視 ※ディフォルト不可
        check_regix_in:String,
        //----- 戻り値 -----//
        ) -> InputParametars { //戻り値の型は構造体InputParametars

        //-------------------------------------------------------------------------- 
        //  正規表現トレイト
        //-------------------------------------------------------------------------- 
        //実数
        let regix_jissuu = Regex::new(r"\d+(?:\.\d+)?").unwrap();
        //整数
        let regix_seisuu = Regex::new(r"[+-]?\d+").unwrap(); 

        //--------------------------------------------------------------------------     
        //  整数の処理 文字列を整数に変換
        //-------------------------------------------------------------------------- 
        let ret_intvalue :i32 =
        if regix_jissuu.is_match(&args) == false {
            0
        } else {
            let _data=regix_seisuu.captures(&args).unwrap().at(0).unwrap();
            let sandata: i32 = _data.parse().expect("変換できない文字列でした");
            sandata
         };

        //--------------------------------------------------------------------------     
        //  文字列長 ※強引にカウント
        //--------------------------------------------------------------------------
         let ret_str_length : i32=args.chars().count() as i32;

        //--------------------------------------------------------------------------     
        //  数値を扱う場合、フロートも算出
        //--------------------------------------------------------------------------
         let ret_float_value  :f64 =
         if string_type_in==true  {
            0.0
         }else if  regix_jissuu.is_match(&args) == false {
            0.0
         } else {
             let _data=regix_jissuu.captures(&args).unwrap().at(0).unwrap();
             let cst_data : f64 = _data.parse().expect("変換できない文字列でした");
             cst_data
        };

        //--------------------------------------------------------------------------
        //  整合性チェックの初期値
        //--------------------------------------------------------------------------
        let mut ret_result: bool = true; //文字列チェック結果の初期値

        //--------------------------------------------------------------------------
        //  文字列長、値のチェック ※存在のみチェックする場合、 0,-1を指定
        //--------------------------------------------------------------------------
        if string_type_in==true   {
            if  ret_str_length <= min_in && min_in >-1  {
                message_in=format!("{}文字より多く入力して下さい", min_in);
                ret_result=false;
            } else  if   ret_str_length > max_in  && max_in > -1 {
                message_in=format!("{}文字以下で入力して下さい", max_in);
                ret_result=false;
            }
        }

        //--------------------------------------------------------------------------
        //  フロート、値のチェック ※-1はマジックナンバー
        //--------------------------------------------------------------------------
        if string_type_in==false   {
            if  ret_float_value <= min_in  as f64 && min_in !=-1  {
                message_in=format!("{}を超える値を入力して下さい", min_in);
                ret_result=false;
            } else  if   ret_float_value > max_in as f64  && max_in != -1 {
                message_in=format!("{}以下の値を入力して下さい", max_in);
                ret_result=false;
            }
        }

        //--------------------------------------------------------------------------
        //  整数、値のチェック ※-1はマジックナンバー
        //--------------------------------------------------------------------------
        if string_type_in==false   {
            if  ret_intvalue <= min_in && min_in !=-1  {
                message_in=format!("{}より多い値を入力して下さい", min_in);
                ret_result=false;
            } else  if   ret_intvalue   > max_in  && max_in != -1 {
                message_in=format!("{}以下の値を入力して下さい", max_in);
                ret_result=false;
            }
        }

        //--------------------------------------------------------------------------
        //  正規表現(境界値精査もこれで実施すること！)
        //--------------------------------------------------------------------------       
        //  値チェックが有効時のみ、結果を反映
        if  ret_result==true {
            ret_result= match &*check_regix_in {
                r"*" => true, 
                _ => {
                    let check_regix = Regex::new(&check_regix_in).unwrap();
                    check_regix.is_match(&args)
                },  
            };
        }

        //--------------------------------------------------------------------------
        // 戻り値として返却
        //--------------------------------------------------------------------------
        InputParametars{
            string_type: string_type_in ,   // 文字列で扱うなら true それ以外なら false
            str_value:args,                       // 文字列でのパンドラ値
            str_length:ret_str_length,               // 文字列長
            int_value:ret_intvalue,           //整数でのパンドラ値
            float_value:ret_float_value,       // floatでのパンドラ値 
            result: ret_result,                           // 値チェックの結果 OK なら true NG なら false
            result_msg : message_in,      // ValidationBack時のメッセージ
        }
    }//メンバ関数:set_input_parametars ブロック
}//トレイト:InputParametars ブロック
