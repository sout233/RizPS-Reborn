use serde::{Deserialize, Serialize};

///SDKLogin的json序列化部分structs

#[derive(Serialize, Deserialize)]
pub(crate) struct GetProduct {
    pub(crate) id: i32,
    pub(crate) costs: Vec<Cost>,
    pub(crate) onSalePercent: f64,
    pub(crate) assets: Vec<Asset>,
    pub(crate) getLimit: i32,
    pub(crate) conditionType: String,
    pub(crate) preTask: Vec<i32>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Cost {
    #[serde(rename = "type")]
    pub(crate) cost_type: String,
    pub(crate) amount: i32
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Asset {
    pub(crate) amount: i32,
    #[serde(rename = "type")]
    pub(crate) asset_type: String,
    pub(crate) assetId: String
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MyBest {
    pub(crate) trackAssetId: String,
    pub(crate) difficultyClassName: String,
    pub(crate) score: i128,
    pub(crate) completeRate: f64,
    pub(crate) isFullCombo: bool,
    pub(crate) isClear: bool
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SDKLogin_JSON {
    pub(crate) _id: String,
    pub(crate) username: String,
    pub(crate) coin: i32,
    pub(crate) dot: i32,
    pub(crate) lastMadeCardId: i32,//这可能得等到RizCard功能实装力...
    pub(crate) getProducts: Vec<GetProduct>,
    pub(crate) myBest: Vec<MyBest>,
    pub(crate) unlockedLevels: Vec<String>,
    pub(crate) appearLevels: Vec<String>
}

//RZPR的accounts json序列化部分

#[derive(Serialize, Deserialize)]
pub(crate) struct RZPR_Accounts {
    pub(crate) sdklogin_username: String,
    pub(crate) sdklogin_coin: i32,
    pub(crate) sdklogin_dot: i32,
    pub(crate) sdklogin_lastmadecardid: i32,
    pub(crate) sdklogin_bests: Vec<MyBest>,
    pub(crate) sdklogin_uklevels: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RZPR_ACJson {
    pub(crate) rzprac_items: Vec<RZPR_Accounts>
}

/*
accounts.rzpr示例：
{
    {
        "sdklogin_username": "abc",
        "sdklogin_coin": 114514,
        "sdklogin_dot": 1919810,
        "sdklogin_lastmadecardid": 0,
        "sdklogin_bests": {...},
        "sdklogin_uklevels": {...}
    }
}
 */