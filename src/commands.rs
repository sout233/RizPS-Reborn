use crate::{get_user_account, structs};
use crate::structs::{RZPR_Accounts, RZPR_ACJson};
use std::fs;

pub fn create_a_sdkchecklogindo_account_no_sdklogin(new_username: &str) -> bool{
    let mut new_user_account_struct = structs::RZPR_Accounts{
        sdklogin_username: (new_username).to_string(),
        sdklogin_gamename: "wait_to_set".to_string(),
        sdklogin_coin: 0,
        sdklogin_dot: 0,
        sdklogin_lastmadecardid: 0,
        sdklogin_bests: Vec::from([]),//新用户那必须得空啊
        sdklogin_uklevels: Vec::from(["track.PastelLines.RekuMochizuki.0".to_string(),"track.Gleam.Uske.0".to_string(),"track.PowerAttack.EBIMAYO.0".to_string()]),//新人三件套
    };
    let mut newacfile = crate::get_serde_accountfile();
    newacfile.rzprac_items.push(new_user_account_struct);
    let json:String = serde_json::to_string(&newacfile).unwrap();
    if(crate::isLogLevelHigh()){
        println!("用户账号注册的反序列化结果：{}",json);
    }
    fs::write("./accounts.rzpr", json);
    true
}

pub fn create_a_sdklogin_account(new_username: &str, new_gamename: &str) -> bool{
    let mut new_user_account_struct = structs::RZPR_Accounts{
        sdklogin_username: (new_username).to_string(),
        sdklogin_gamename: new_gamename.to_string(),
        sdklogin_coin: 0,
        sdklogin_dot: 0,
        sdklogin_lastmadecardid: 0,
        sdklogin_bests: Vec::from([]),
        sdklogin_uklevels: Vec::from(["track.PastelLines.RekuMochizuki.0".to_string(),"track.Gleam.Uske.0".to_string(),"track.PowerAttack.EBIMAYO.0".to_string()]),
    };
    let mut newacfile = crate::get_serde_accountfile();
    newacfile.rzprac_items.push(new_user_account_struct);
    let json:String = serde_json::to_string(&newacfile).unwrap();
    fs::write("./accounts.rzpr", json);
    true
}

pub fn change_gamename(mut acjson: RZPR_ACJson, target_username: String, new_gamename: String) -> bool{
    if let Some(rzpr_account) = acjson.rzprac_items.iter().find(|rzpr_account| rzpr_account.sdklogin_username == target_username) {//iter_mut可以让其变得能够被修改
        let mut old_account_struct_with_old_gamename = rzpr_account.to_owned();
        let mut new_account_struct = rzpr_account.to_owned();
        new_account_struct.sdklogin_gamename = new_gamename.to_string();
        let mut newacfile = crate::get_serde_accountfile();
        let (index, _) = acjson.rzprac_items.clone().iter().enumerate().find(|(_, x)| *x.sdklogin_username == target_username).unwrap(); //找到旧的rzpr_accounts的索引
        newacfile.rzprac_items.remove(index);
        newacfile.rzprac_items.push(new_account_struct.to_owned());
        let json:String = serde_json::to_string(&newacfile).unwrap();
        fs::write("./accounts.rzpr", json);
        return true
    }
    else{
        return false
    }
}