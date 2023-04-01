mod structs;
mod commands;

use axum::{
    routing::any,
    http::{header::{HeaderMap, HeaderName, HeaderValue},header},
    response::Json,
    Router,
};
use std::{
    path::Path,
    fs,
    time::{SystemTime, UNIX_EPOCH},collections::HashMap
};
use crypto::{
    md5::Md5,
    digest::Digest
};
use openssl::rsa::{Rsa, Padding};
use colored::Colorize;
use axum_server::tls_rustls::RustlsConfig;
use structs::{SDKLogin_JSON};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::iter::Iterator;
use axum::http::StatusCode;
use serde_json::Value::Null;
use crate::commands::{change_gamename, create_a_sdkchecklogindo_account_no_sdklogin};
use crate::structs::{PostBody_SDKLogin, RZPR_Accounts, RZPR_ACJson};

//一些通用的工具函数

pub fn rsa_private_encrypt(content: &str, private_key: &str) -> String{
    println!("{} -> 准备加密的MD5：{content}","SDKLogin.RSAEncrypt".bright_yellow());
    let private_key = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
    let mut buf = vec![0; private_key.size() as usize];
    let enc_data = private_key.private_encrypt(content.as_bytes(),&mut buf,Padding::PKCS1).unwrap();
    let b64_enc_data: String = base64::encode(buf);
    println!("{} -> 已完成对明文的MD5加密","SDKLogin.RSAEncrypt".bright_yellow());
    b64_enc_data
}

pub fn aes_encrypt(key: &str, iv: String, data: &str) -> String {
    println!("{} -> 准备对明文进行AES加密","SDKLogin.AESEncrypt".bright_yellow());
    let aes_encrypt_result = openssl::symm::encrypt(openssl::symm::Cipher::aes_256_cbc(), key.as_bytes(), Some(iv.as_bytes()), data.as_bytes()).unwrap();
    let b64_enc_data: String = base64::encode(aes_encrypt_result);
    println!("{} -> 已完成对明文的AES加密","SDKLogin.AESEncrypt".bright_yellow());
    b64_enc_data
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn get_user_account(ac_struct: RZPR_ACJson, username: String) -> RZPR_Accounts {
    ac_struct.rzprac_items.iter().find(|item| item.sdklogin_username == username).cloned().unwrap_or_else(|| RZPR_Accounts{
        sdklogin_username: "Not Found This Account".to_string(),
        sdklogin_gamename: "Not Found".to_string(),
        sdklogin_coin: 0,
        sdklogin_dot: 0,
        sdklogin_lastmadecardid: 0,
        sdklogin_bests: vec![],
        sdklogin_uklevels: vec![],
    })
}

pub fn is_user_exists(username: String) -> bool{
    let accounts: structs::RZPR_ACJson = get_serde_accountfile();
    if accounts.rzprac_items.iter().any(|item| item.sdklogin_username == username) {
        if(isLogLevelHigh()){
            println!("is_user_exists: 用户{}存在",username);
        }
        true
    } else {
        if(isLogLevelHigh()){
            println!("is_user_exists: 用户{}不存在",username);
        }
        false
    }
}

pub fn is_user_set_gamename(username: String) -> bool{
    let accounts: structs::RZPR_ACJson = get_serde_accountfile();
    if (get_user_account(accounts,username.clone()).sdklogin_gamename != "wait_to_set") {
        if(isLogLevelHigh()){
            println!("is_user_set_gamename: 用户{}已经设置过gamename了",username);
        }
        true
    } else {
        if(isLogLevelHigh()){
            println!("is_user_set_gamename: 用户{}未设置过gamename",username);
        }
        false
    }
}

pub fn get_serde_accountfile() -> RZPR_ACJson{
    let account_json = fs::File::open("./accounts.rzpr").unwrap();
    let accounts: structs::RZPR_ACJson = serde_json::from_reader(account_json).unwrap();
    accounts
}

pub fn get_serde_basesdklogin() -> SDKLogin_JSON{
    let sdkl_json = fs::File::open("./SDKLogin.json").unwrap();
    let sdklogin_serde: structs::SDKLogin_JSON = serde_json::from_reader(sdkl_json).unwrap();
    sdklogin_serde
}

pub fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn isLogLevelHigh() -> bool{
    let server_conf_file = fs::File::open("./config.json").unwrap();
    let server_conf: serde_json::Value = serde_json::from_reader(server_conf_file).unwrap();
    if(server_conf["output"]["loglevel"].to_string().replace("\"","") == "1"){
        true
    }
    else{
        false
    }
}

//http请求处理函数部分

async fn get_root() -> (HeaderMap, &'static str){
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/html")
    );
    (headers,"<meta charset=\"utf-8\">Welcome To RizPS-Reborn<br/>如果你能够看到这段话，则代表RizPS-Reborn运行正常，开始畅玩吧！")
}//get根目录时的返回

//410001系列初始化请求处理

async fn ltsdk_410001_20190403() -> String{
    println!("GET -> 客户端正在请求410001_config_20190403.json");
    let read_result: String = fs::read_to_string("./req_files/410001_config_20190403.json").unwrap();
    read_result
}

async fn broken_ios_official_client() -> String{
    println!("{} -> 来自非RizPS-Reborn定制客户端的连接","BROKEN".purple());
    let read_result: String = fs::read_to_string("./req_files/410001_main_dis_block.json").unwrap();
    read_result
}

async fn broken_android_official_client() -> String{
    println!("{} -> 来自非RizPS-Reborn定制客户端的连接","BROKEN".purple());
    let read_result: String = fs::read_to_string("./req_files/310001_main_dis_block.json").unwrap();
    read_result
}

async fn connect_ios_custom_client() -> String{
    println!("{} -> 来自RizPS-Reborn定制客户端的连接","CONNECTED".cyan());
    let read_result: String = fs::read_to_string("./req_files/410001_main_dis.json").unwrap();
    read_result
}

async fn connect_android_custom_client() -> String{
    println!("{} -> 来自RizPS-Reborn定制客户端的连接","CONNECTED".cyan());
    let read_result: String = fs::read_to_string("./req_files/310001_main_dis.json").unwrap();
    read_result
}

//SDK必要请求

async fn sdk_language_config() -> String{
    let read_result: String = fs::read_to_string("./req_files/languageConfig.json").unwrap();
    read_result
}

async fn sdk_ExceptionTrack() -> String{
    let timestamp_now : String = (SystemTime::now().duration_since(UNIX_EPOCH)).unwrap().as_secs().to_string();
    let ret : String = "{\"flag\":true,\"code\":0,\"desc\":\"\",\"time\":".to_string() + &timestamp_now + &",\"data\":false}".to_string();
    ret
}

async fn sdk_api_rules() -> String{
    "{\"message\":\"invalid signature\"}".to_string()
}

async fn sdk_initset() -> String{
    "{\"flag\":true,\"code\":0,\"desc\":\"\",\"data\":true}".to_string()
}

async fn sdk_lang_zh() -> String{
    let read_result: String = fs::read_to_string("./req_files/zh.json").unwrap();
    read_result
}

async fn sdk_lang_zhk() -> String{
    let read_result: String = fs::read_to_string("./req_files/zhhk.json").unwrap();
    read_result
}

async fn sdk_lang_zhtw() -> String{
    let read_result: String = fs::read_to_string("./req_files/zhtw.json").unwrap();
    read_result
}

//游戏本体请求处理部分

async fn InGameErrorReport(Json(errinfo) : Json<serde_json::Value>) -> String{
    println!("{} -> 客户端疑似报错！错误信息：{errinfo}","CLIENT.ERROR".red());
    "success".to_string()
}

async fn GuestLogin_DO() -> String{
    println!("{} -> 客户端正在尝试注册账号","GUESTLOGIN.DO".yellow());
    let timestamp_now : String = (SystemTime::now().duration_since(UNIX_EPOCH)).unwrap().as_secs().to_string();
    let give_new_username: &str = &*generate_random_string(12);
    create_a_sdkchecklogindo_account_no_sdklogin(give_new_username);
    let ret: String = "{\"message\":\"{\\\"timestamp\\\":\\\"".to_string() + &timestamp_now + &"\\\",\\\"sid\\\":\\\""+ give_new_username + &"\\\",\\\"warnEndDate\\\":null,\\\"token\\\":\\\"我是翟健，我被困在无人的大沙漠里了，我现在需要钱来支撑我出去，V我50，出去后听我讲述我对私服的报仇计划\\\",\\\"cmtBirth\\\":\\\"9\\\",\\\"bind\\\":\\\"9\\\"}\",\"status\":\"1\"}".to_string();
    ret
}

async fn SDKLogin_DO(post_body: String) -> String{
    println!("{} -> 客户端正在尝试使用SDK登录","SDKLOGIN.DO".yellow());
    let iter = post_body.split('&');
    // 把迭代器转换成一个&str类型的向量
    let postbody_v: Vec<&str> = iter.collect();
    if(is_user_exists(postbody_v.get(0).unwrap().to_string())) {
        let timestamp_now: String = (SystemTime::now().duration_since(UNIX_EPOCH)).unwrap().as_secs().to_string();
        let ret: String = "{\"message\":\"{\\\"timestamp\\\":\\\"".to_string() + &timestamp_now + &"\\\",\\\"warnEndDate\\\":null,\\\"token\\\":\\\"什么，这不是饼干，这是RizPS-Reborn！我们这个RizPS-Reborn体积小方便携带，拆开一包，放水里就变大，怎么扯都扯不坏，用来嫖鸽游，夜袭CN115，惹惹翟健，都是很好用的。你看解压以后比Grasscutter还小，放在水里遇水变大变高，吸水性很强的。解压以后，是一只四肢健全的RizPS-Reborn，你看他怎么擦都擦不坏，好不掉毛不掉絮，使用七八次都没问题，出差旅行带上它非常方便，用它SDKCheckLogin.do，再SDKLogin，AESEncrypt，干净卫生。什么?在哪里买?下方Gayhub，买五包送五包，还包邮 Powered By 矮人科技\\\",\\\"priority\\\":0,\\\"cmtBirth\\\":\\\"9\\\",\\\"bind\\\":\\\"9\\\"}\",\"status\":\"1\"}".to_string();
    }
    else{
        ret = "{\"message\":\"username or password error\",\"status\":\"10001\"}";
    }
    ret
}

async fn SDKLogin(Json(post_body) : Json<structs::PostBody_SDKLogin>) -> (StatusCode,HeaderMap, String){
    println!("{} -> 客户端正在尝试下载存档数据","SDKLOGIN".yellow());
    let mut sdklogin_hasher = Md5::new();
    let mut sdklogin_serde = get_serde_basesdklogin();
    let mut ac_serde = get_user_account(get_serde_accountfile(),post_body.username.clone());
    println!("ac_serde.sdklogin_gamename: {}",ac_serde.sdklogin_gamename);
    sdklogin_serde.username = ac_serde.sdklogin_gamename + "#" + &*ac_serde.sdklogin_username;//读取并设置gamename与username
    sdklogin_serde.coin = ac_serde.sdklogin_coin;
    sdklogin_serde.dot = ac_serde.sdklogin_dot;
    sdklogin_serde.myBest = ac_serde.sdklogin_bests;
    sdklogin_serde.unlockedLevels = ac_serde.sdklogin_uklevels;
    let mut userid_clone: String = post_body.userId.clone();
    if(is_user_set_gamename( userid_clone.clone()) && is_user_exists(userid_clone.clone())){
        let origin_text = String::from(serde_json::to_string(&sdklogin_serde).unwrap());
        sdklogin_hasher.input_str(&origin_text);
        let rsa_signed: String = rsa_private_encrypt(sdklogin_hasher.result_str().as_str(), &fs::read_to_string("./RizPS-Reborn-Custom-RSA-Keys/private.pem").unwrap());
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("sign"),
            HeaderValue::from_static(string_to_static_str(rsa_signed))
        );
        headers.insert(
            HeaderName::from_static("set-token"),
            HeaderValue::from_str(userid_clone.as_str()).unwrap()
        );
        return (StatusCode::OK,headers, aes_encrypt("Sv@H,+SV-U*VEjCW,n7WA-@n}j3;U;XF", "1%[OB.<YSw?)o:rQ".to_string(), fs::read_to_string("./SDKLogin.json").unwrap().as_str()))
    }
    let mut headers = HeaderMap::new();
    return (StatusCode::NOT_FOUND,headers, "{\"message\":\"该用户尚未注册\",\"code\":1}".to_string())
}

async fn SDKRegister(Json(post_body) : Json<structs::PostBody_SDKLogin>) -> (StatusCode,HeaderMap, String){
    println!("{} -> 客户端正在尝试注册游戏账号","SDKREGISTER".yellow());
    if(!is_user_exists( post_body.userId.clone())) {
        return (StatusCode::BAD_REQUEST, HeaderMap::new(), "{\"message\":\"这个账号不存在，可能是因为没过guestLogin.do，尝试重装游戏或更新RizPS-Reborn？\",\"code\":1}".to_string())
    }
    if(is_user_set_gamename( post_body.userId.clone())) {
        return (StatusCode::BAD_REQUEST, HeaderMap::new(), "{\"message\":\"这个账号已经被注册，完全可以直接使用/SDKLogin进行请求登录，但客户端扔发送了/SDKRegister请求进行用户重命名与注册，尝试重装游戏？\",\"code\":1}".to_string())
    }
    change_gamename(get_serde_accountfile(),post_body.userId.clone(),post_body.username.clone());
    let mut sdklogin_hasher = Md5::new();
    let mut sdklogin_serde: SDKLogin_JSON = get_serde_basesdklogin();
    let mut ac_serde = get_user_account(get_serde_accountfile(),post_body.username.clone());
    sdklogin_serde.username = ac_serde.sdklogin_gamename + "#" + &*ac_serde.sdklogin_username;//读取并设置gamename与username
    sdklogin_serde.coin = ac_serde.sdklogin_coin;
    sdklogin_serde.dot = ac_serde.sdklogin_dot;
    sdklogin_serde.myBest = ac_serde.sdklogin_bests;
    sdklogin_serde.unlockedLevels = ac_serde.sdklogin_uklevels;
    let userid_clone = post_body.userId;
    let origin_text = String::from(serde_json::to_string(&sdklogin_serde).unwrap());
    sdklogin_hasher.input_str(&origin_text);
    let rsa_signed: String = rsa_private_encrypt(sdklogin_hasher.result_str().as_str(), &fs::read_to_string("./RizPS-Reborn-Custom-RSA-Keys/private.pem").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("sign"),
        HeaderValue::from_static(string_to_static_str(rsa_signed))
    );
    headers.insert(
        HeaderName::from_static("set-token"),
        HeaderValue::from_str(userid_clone.as_str()).unwrap()
    );
    return (StatusCode::OK,headers, aes_encrypt("Sv@H,+SV-U*VEjCW,n7WA-@n}j3;U;XF", "1%[OB.<YSw?)o:rQ".to_string(), fs::read_to_string("./SDKLogin.json").unwrap().as_str()))
}

async fn afterplay() -> (HeaderMap, String){
    println!("{} -> 客户端打完了一首歌","AFTER.PLAY".yellow());
    let mut sdklogin_hasher = Md5::new();
    let origin_text = String::from("{\"data\": \"idk\"}");
    sdklogin_hasher.input_str(&origin_text);
    let rsa_signed: String = rsa_private_encrypt(sdklogin_hasher.result_str().as_str(), &fs::read_to_string("./RizPS-Reborn-Custom-RSA-Keys/private.pem").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("sign"),
        HeaderValue::from_static(string_to_static_str(rsa_signed))
    );
    (headers, aes_encrypt("Sv@H,+SV-U*VEjCW,n7WA-@n}j3;U;XF", "1%[OB.<YSw?)o:rQ".to_string(), "{\"data\": \"idk\"}"))
}

async fn NetWorkTest() -> &'static str{
    "success"
} 

async fn resources_download(axum::extract::Path(down_url): axum::extract::Path<HashMap<String, String>>) -> Vec<u8>{
    let mut req_file_path: String = "./resources/".to_string() + down_url.get("platform").unwrap() + &"/".to_string() + down_url.get("file").unwrap();
    println!("{} -> 请求平台：{req_platform} 文件：{req_file_url}","Resource.Download".purple(), req_platform = down_url.get("platform").unwrap(), req_file_url = down_url.get("file").unwrap());
    let read_content = fs::read(req_file_path).unwrap();
    read_content
}

async fn songs_download(axum::extract::Path(down_url): axum::extract::Path<HashMap<String, String>>) -> Vec<u8>{
    let mut req_file_path: String = "./resources/".to_string() + down_url.get("platform").unwrap() + &"/".to_string() + down_url.get("req_file_no_bundle").unwrap() + &".bundle".to_string();
    println!("{} -> 请求平台：{req_platform} 文件：{req_file_url}","Songs/Sheets.Download".purple(), req_platform = down_url.get("platform").unwrap(), req_file_url = down_url.get("req_file_no_bundle").unwrap());
    println!("{req_file_path}");
    let read_content = fs::read(req_file_path).unwrap();
    read_content
}

async fn logback() -> (HeaderMap, String){
    let mut logback_hasher = Md5::new();
    let origin_text = String::from("{\"data\": \"idk\"}");
    logback_hasher.input_str(&origin_text);
    let rsa_signed: String = rsa_private_encrypt(logback_hasher.result_str().as_str(), &fs::read_to_string("./RizPS-Reborn-Custom-RSA-Keys/private.pem").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("sign"),
        HeaderValue::from_static(string_to_static_str(rsa_signed))
    );
    (headers, aes_encrypt("Sv@H,+SV-U*VEjCW,n7WA-@n}j3;U;XF", "1%[OB.<YSw?)o:rQ".to_string(), "success"))
}

//为后端运营面板或各种插件提供的接口

async fn get_test() -> &'static str{
    "ok"
}//测试服务器是否在线用的get返回

async fn get_ios_shadowsocks_conf() -> String{
    let read_result: String = fs::read_to_string("./req_files/ios_shadowrockets_conf.conf").unwrap();
    read_result
}

#[tokio::main]
async fn main() {
    println!("\n- {} -\nRizPS-Reborn是免费且永久开源的软件，并遵循GPL-3开源协议，这意味着你若要发布修改后的RizPS-Reborn，则必须同时开源。如果你是通过购买的方式得到了该软件，那么这代表你已经被骗了，请给店家差评并申请退款。\n感谢任何对此项目提出建议/报告问题/贡献代码的人，我爱你们！\n","RizPS-Reborn v1.0.2".bright_blue());

    if(!Path::new("./req_files").exists()){
        println!("{} -> req_files文件夹不存在，无法在此文件夹不存在的情况下继续维持RizPS-Reborn的运行，结束运行！","SERVER.INIT.ERROR".red());
        std::process::exit(101);
    }
    else if(!Path::new("./SDKLogin.json").exists()){
        println!("{} -> SDKLogin.json不存在，无法在此文件不存在的情况下继续维持RizPS-Reborn的运行，结束运行！","SERVER.INIT.ERROR".red());
        std::process::exit(101);
    }//RizPS-Reborn完整性校验

    /*
    if(!Path::new("./resources/Android/catalog_catalog.hash").exists()){
        println!("{} -> resources文件夹不存在或内容不完整，如果你打算离线游玩（使用FiddlerScript.cs），在游玩时可能会出现大量报错以及无法下载更新和歌曲/铺面。若您并未拥有resources，请前往RizPS-Reborn的Github Releases页面中下载。若您是在线游玩（使用FiddlerScriptOnline.cs），请忽视","SERVER.INIT.WARNING".bright_yellow())
    }//res校验
    */
    //我为什么砍res功能？首先你先别急，然后你再别急

    if(!Path::new("./config.json").exists()){
        println!("{} -> 配置文件 (./config.json) 不存在，正在尝试创建...","SERVER.INIT".blue());
        fs::write("./config.json", "{\"server\": {\"ip\": \"0.0.0.0\",\"port\": \"443\",\"web_panel\":\"true\",\"web_panel_ip\":\"0.0.0.0\",\"web_panel_port\":\"1275\"},\"output\": {\"loglevel\": \"0\"}}");
    }
    else{
        println!("{} -> 配置文件存在，启动服务器~","SERVER.INIT".green())
    }//配置文件检查

    if(!Path::new("./accounts.rzpr").exists()){
        println!("{} -> 账号数据文件 (./accounts.rzpr) 不存在，正在尝试创建...","SERVER.INIT".blue());
        fs::write("./accounts.rzpr", "{\"rzprac_items\": [{\"sdklogin_username\": \"rzpusers\",\"sdklogin_gamename\": \"通用账号\",\"sdklogin_coin\": 114514,\"sdklogin_dot\": 1919810,\"sdklogin_lastmadecardid\": 0,\"sdklogin_bests\": [],\"sdklogin_uklevels\": [\"track.PastelLines.RekuMochizuki.0\",\"track.Gleam.Uske.0\",\"track.PowerAttack.EBIMAYO.0\"]}]}");
    }
    else{
        println!("{} -> 配置文件存在，启动服务器~","SERVER.INIT".green())
    }//accounts文件检查
    
    //读配置文件
    let server_conf_file = fs::File::open("./config.json").unwrap();
    let server_conf: serde_json::Value = serde_json::from_reader(server_conf_file).unwrap();
    let mut want_to_exit: i64 = 0;

    println!("{} -> 配置文件读取成功，数据：{server_conf}","SERVER.INIT".green());

    //创建app并进行route绑定
    let app = Router::new()
        .route("/", any(get_root))
        .route("/67/410001_config_20190403.json", any(ltsdk_410001_20190403))
        .route("/lvdgj/version/release/410001_main.dis", any(broken_ios_official_client))
        .route("/lvdgj/version/release/310001_main.dis", any(broken_android_official_client))
        .route("/lvdgj/version/release/410001_rizps.is", any(connect_ios_custom_client))
        .route("/lvdgj/version/release/310001_rizps.is", any(connect_android_custom_client))
        .route("/language/languageConfig.json", any(sdk_language_config))
        .route("/elva/api/SdkTrack/ExceptionTrack", any(sdk_ExceptionTrack))
        .route("/api/v1.0/rules", any(sdk_api_rules))
        .route("/elva/api/initset", any(sdk_initset))
        .route("/language/language/zh-CN.json", any(sdk_lang_zh))
        .route("/language/language/zh-HK.json", any(sdk_lang_zhk))
        .route("/language/language/zh-TW.json", any(sdk_lang_zhtw))
        .route("/log/chargeLogReport.do", any(InGameErrorReport))
        .route("/login/guestLogin.do", any(GuestLogin_DO))
        .route("/login/sdkCheckLogin.do", any(SDKLogin_DO))
        .route("/SDKLogin", any(SDKLogin))
        .route("/SDKRegister", any(SDKRegister))
        .route("/after_play",any(afterplay))
        .route("/isc", any(get_ios_shadowsocks_conf))
        .route("/test", any(NetWorkTest))
        .route("/logBack",any(logback))//在切屏后返回rizline时请求，不响应游戏会寄
        .route("/testasset/:platform/:file", any(resources_download))
        .route("/songsdata/:platform/cridata_assets_criaddressables/:req_file_no_bundle", any(songs_download))
        .route("/checklive", any(get_test));
 
    ctrlc::set_handler(move || {
        if(want_to_exit == 0){
            println!("{}","-> 真的要退出RizPS-Reborn吗？再按一次Ctrl+C以退出 <-".red());
            want_to_exit += 1;
        }
        else{
            std::process::exit(0)
        }
    }).expect("Error setting Ctrl-C handler");
    
    //既傻逼又屎山的代码，由于使用Value解析json导致key对应的内容带双引号，直接replace掉曲线救国🤣
    //我去，把我自己都整乐了
    let mut addr_with_port: String = server_conf["server"]["ip"].to_string().replace("\"", "") + &":" + &server_conf["server"]["port"].to_string().replace("\"", "");
    println!("{} -> 高日志等级：{}","SERVER.INIT".green(),isLogLevelHigh());
    if(isLogLevelHigh()){
        println!("{} -> 日志等级为高，这可能会导致一条条巨长无比的log向你袭来，如果不是为了开发调试，请不要使用高日志等级，这不仅会让问题变得难以排查，还会给服务器造成不必要的压力","SERVER.WARN".yellow());
    }
    println!("{} -> 服务器将在https://{addr_with_port}上启动~ 注意，是HTTPS而非HTTP!","SERVER.INIT".green());

    let tls_config = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem"
    )
    .await
    .unwrap();//配置证书相关 如果证书没了可以这样生成：openssl req -x509 -newkey rsa:4096 -sha256 -nodes -keyout key.pem -out cert.pem -days 114514 前提是你有openssl

    //开服
    axum_server::bind_rustls(addr_with_port.parse().unwrap(), tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("{} -> 服务器被终止","SERVER.CLOSE".red());//实际上在用户Ctrl+C后这段文字并不会被输出，但是谁知道呢？
}