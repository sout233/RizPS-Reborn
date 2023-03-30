use serde::{Deserialize, Serialize};

///SDKLogin的json序列化部分structs

#[derive(Serialize, Deserialize)]
struct GetProduct {
    id: i32,
    costs: Vec<Cost>,
    onSalePercent: f64,
    assets: Vec<Asset>,
    getLimit: i32,
    conditionType: String,
    preTask: Vec<i32>
}

#[derive(Serialize, Deserialize)]
struct Cost {
    #[serde(rename = "type")]
    cost_type: String,
    amount: i32
}

#[derive(Serialize, Deserialize)]
struct Asset {
    amount: i32,
    #[serde(rename = "type")]
    asset_type: String,
    assetId: String
}

#[derive(Serialize, Deserialize)]
struct MyBest {
    trackAssetId: String,
    difficultyClassName: String,
    score: i128,
    completeRate: f64,
    isFullCombo: bool,
    isClear: bool
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SDKLogin_JSON {
    _id: String,
    username: String,
    coin: i32,
    dot: i32,
    lastMadeCardId: i32,//这可能得等到RizCard功能实装力...
    getProducts: Vec<GetProduct>,
    myBest: Vec<MyBest>,
    unlockedLevels: Vec<String>,
    appearLevels: Vec<String>
}

//RZPR的accounts json序列化部分

#[derive(Serialize, Deserialize)]
struct RZPR_Accounts {
    sdklogin_username: String,
    sdklogin_coin: i32,
    sdklogin_dot: i32,
    sdklogin_lastmadecardid: i32,
    sdklogin_bests: Vec<MyBest>,
    sdklogin_uklevels: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RZPR_ACJson {
    rzprac_items: Vec<RZPR_Accounts>
}