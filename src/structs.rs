use serde::{Deserialize, Serialize};

//SDKLogin客户端POST内容部分structs
#[derive(Serialize, Deserialize)]
pub struct PostBody_SDKLogin {
    userId: String,
    game: String,
    channelNo: String,
    token: String,
    username: String
}

///SDKLogin的json序列化部分structs

#[derive(Serialize, Deserialize)]
pub struct GetProduct {
    pub id: i32,
    pub costs: Vec<Cost>,
    pub onSalePercent: f64,
    pub assets: Vec<Asset>,
    pub getLimit: i32,
    pub conditionType: String,
    pub preTask: Vec<i32>
}

#[derive(Serialize, Deserialize)]
pub struct Cost {
    #[serde(rename = "type")]
    pub cost_type: String,
    pub amount: i32
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    pub amount: i32,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub assetId: String
}

#[derive(Serialize, Deserialize)]
pub struct MyBest {
    pub trackAssetId: String,
    pub difficultyClassName: String,
    pub score: i128,
    pub completeRate: f64,
    pub isFullCombo: bool,
    pub isClear: bool
}

#[derive(Serialize, Deserialize)]
pub struct SDKLogin_JSON {
    pub _id: String,
    pub username: String,
    pub coin: i32,
    pub dot: i32,
    pub lastMadeCardId: i32,//这可能得等到RizCard功能实装力...
    pub getProducts: Vec<GetProduct>,
    pub myBest: Vec<MyBest>,
    pub unlockedLevels: Vec<String>,
    pub appearLevels: Vec<String>
}

//RZPR的accounts json序列化部分

#[derive(Serialize, Deserialize)]
pub  struct RZPR_Accounts {
    pub sdklogin_username: String,
    pub sdklogin_coin: i32,
    pub sdklogin_dot: i32,
    pub sdklogin_lastmadecardid: i32,
    pub sdklogin_bests: Vec<MyBest>,
    pub sdklogin_uklevels: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct RZPR_ACJson {
    pub rzprac_items: Vec<RZPR_Accounts>
}

/*
accounts.rzpr示例：
{
    "rzprac_items": {
        "sdklogin_username": "abc",
        "sdklogin_coin": 114514,
        "sdklogin_dot": 1919810,
        "sdklogin_lastmadecardid": 0,
        "sdklogin_bests": {...},
        "sdklogin_uklevels": {...}
    }
}
 */