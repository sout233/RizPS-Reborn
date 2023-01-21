use axum::{
    routing::get,
    routing::post,
    routing::any,
    http::{Request, header::{HeaderMap, HeaderName, HeaderValue},header},
    response::{IntoResponse, Html, Json},
    Router,
};
use std::{
    path::Path,
    fs,
    time::{SystemTime, UNIX_EPOCH, Duration}, str::FromStr, hash::Hash, collections::HashMap
};
use crypto::{
    md5::Md5,
    digest::Digest
};
use openssl::rsa::{Rsa, Padding};
use openssl::hash::MessageDigest;
use crypto::sha2::Sha256;
use std::iter::repeat;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;
use colored::Colorize;
use axum_server::tls_rustls::RustlsConfig;
use openssl::aes::{AesKey, aes_ige};

type AesCbc = Cbc<Aes256, Pkcs7>;

pub fn rsa_private_encrypt(content: &str, private_key: &str) -> String{
    println!("{} -> 准备加密的MD5：{content}","SDKLogin.RSAEncrypt".bright_yellow());
    let mut rng = rand::thread_rng();
    let private_key = Rsa::private_key_from_pem(private_key.as_bytes()).unwrap();
    let mut buf = vec![0; private_key.size() as usize];
    let enc_data = private_key.private_encrypt(content.as_bytes(),&mut buf,Padding::PKCS1).unwrap();
    let b64_enc_data: String = base64::encode(buf);
    println!("{} -> 已完成对SDKLogin.json的MD5加密","SDKLogin.RSAEncrypt".bright_yellow());
    b64_enc_data
}

pub fn aes_encrypt(key: &str, iv: String, data: &str) -> String {
    println!("{} -> 准备对SDKLogin.json进行AES加密","SDKLogin.AESEncrypt".bright_yellow());
    let aes_encrypt_result = openssl::symm::encrypt(openssl::symm::Cipher::aes_256_cbc(), key.as_bytes(), Some(iv.as_bytes()), data.as_bytes()).unwrap();
    let b64_enc_data: String = base64::encode(aes_encrypt_result);
    println!("{} -> 已完成对SDKLogin.json的AES加密","SDKLogin.AESEncrypt".bright_yellow());
    b64_enc_data
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
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

//游戏catalog与catalog hash，更新离线必须

async fn ret_catalog() -> String{
    let read_result: String = fs::read_to_string("./req_files/riz102-catalog.json").unwrap();
    read_result
}

async fn ret_catalog_hash() -> String{
    let read_result: String = fs::read_to_string("./req_files/riz102-catalog-hash.txt").unwrap();
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
    let ret: String = "{\"message\":\"{\\\"timestamp\\\":\\\"".to_string() + &timestamp_now + &"\\\",\\\"sid\\\":\\\"rzpusers\\\",\\\"warnEndDate\\\":null,\\\"token\\\":\\\"我是翟健，我被困在无人的大沙漠里了，我现在需要钱来支撑我出去，V我50，出去后听我讲述我对私服的报仇计划\\\",\\\"cmtBirth\\\":\\\"9\\\",\\\"bind\\\":\\\"9\\\"}\",\"status\":\"1\"}".to_string();
    ret
}

async fn SDKLogin_DO() -> String{
    println!("{} -> 客户端正在尝试使用SDK登录","SDKLOGIN.DO".yellow());
    let timestamp_now : String = (SystemTime::now().duration_since(UNIX_EPOCH)).unwrap().as_secs().to_string();
    let ret: String = "{\"message\":\"{\\\"timestamp\\\":\\\"".to_string() + &timestamp_now + &"\\\",\\\"warnEndDate\\\":null,\\\"token\\\":\\\"什么，这不是饼干，这是RizPS-Reborn！我们这个RizPS-Reborn体积小方便携带，拆开一包，放水里就变大，怎么扯都扯不坏，用来嫖鸽游，夜袭CN115，惹惹翟健，都是很好用的。你看解压以后比Grasscutter还小，放在水里遇水变大变高，吸水性很强的。解压以后，是一只四肢健全的RizPS-Reborn，你看他怎么擦都擦不坏，好不掉毛不掉絮，使用七八次都没问题，出差旅行带上它非常方便，用它SDKCheckLogin.do，再SDKLogin，AESEncrypt，干净卫生。什么?在哪里买?下方Gayhub，买五包送五包，还包邮\\\",\\\"priority\\\":0,\\\"cmtBirth\\\":\\\"9\\\",\\\"bind\\\":\\\"9\\\"}\",\"status\":\"1\"}".to_string();
    ret
}

async fn SDKLogin() -> (HeaderMap, String){
    println!("{} -> 客户端正在尝试下载存档数据","SDKLOGIN".yellow());
    let mut sdklogin_hasher = Md5::new();
    let origin_text = String::from(fs::read_to_string("./SDKLogin.json").unwrap());
    sdklogin_hasher.input_str(&origin_text);
    let rsa_signed: String = rsa_private_encrypt(sdklogin_hasher.result_str().as_str(), &fs::read_to_string("./RizPS-Reborn-Custom-RSA-Keys/private.pem").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("sign"),
        HeaderValue::from_static(string_to_static_str(rsa_signed))
    );
    (headers, aes_encrypt("Sv@H,+SV-U*VEjCW,n7WA-@n}j3;U;XF", "1%[OB.<YSw?)o:rQ".to_string(), fs::read_to_string("./SDKLogin.json").unwrap().as_str()))
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
    println!("\n- {} -\nRizPS-Reborn是免费且永久开源的软件，并遵循GPL-3开源协议，这意味着你若要发布修改后的RizPS-Reborn，则必须同时开源。如果你是通过购买的方式得到了该软件，那么这代表你已经被骗了，请给店家差评并申请退款。\n感谢任何对此项目提出建议/报告问题/贡献代码的人，我爱你们！\n","RizPS-Reborn v1.0.1".bright_blue());

    if(!Path::new("./req_files").exists()){
        println!("{} -> req_files文件夹不存在，无法在此文件夹不存在的情况下继续维持RizPS-Reborn的运行，结束运行！","SERVER.INIT.ERROR".red());
        std::process::exit(101);
    }
    else if(!Path::new("./SDKLogin.json").exists()){
        println!("{} -> SDKLogin.json不存在，无法在此文件不存在的情况下继续维持RizPS-Reborn的运行，结束运行！","SERVER.INIT.ERROR".red());
        std::process::exit(101);
    }//RizPS-Reborn完整性校验

    if(!Path::new("./resources/Android/catalog_catalog.hash").exists()){
        println!("{} -> resources文件夹不存在或内容不完整，如果你打算离线游玩（使用FiddlerScript.cs），在游玩时可能会出现大量报错以及无法下载更新和歌曲/铺面。若您并未拥有resources，请前往RizPS-Reborn的Github Releases页面中下载。若您是在线游玩（使用FiddlerScriptOnline.cs），请忽视","SERVER.INIT.WARNING".bright_yellow())
    }//res校验

    if(!Path::new("./config.json").exists()){
        println!("{} -> 配置文件 (./config.json) 不存在，正在尝试创建...","SERVER.INIT".blue());
        fs::write("./config.json", "{\"server\": {\"ip\": \"0.0.0.0\",\"port\": \"443\"},\"output\": {\"loglevel\": \"0\"}}");
    }
    else{
        println!("{} -> 配置文件存在，启动服务器~","SERVER.INIT".green())
    }//配置文件检查
    
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
        .route("/testasset/iOS/catalog_catalog.json", any(ret_catalog))//仅离线更新功能需要，平时用不到
        .route("/testasset/iOS/catalog_catalog.hash", any(ret_catalog_hash))//这个也是仅离线更新才用得到
        .route("/language/language/zh-CN.json", any(sdk_lang_zh))
        .route("/language/language/zh-HK.json", any(sdk_lang_zhk))
        .route("/language/language/zh-TW.json", any(sdk_lang_zhtw))
        .route("/log/chargeLogReport.do", any(InGameErrorReport))
        .route("/login/guestLogin.do", any(GuestLogin_DO))
        .route("/login/sdkCheckLogin.do", any(SDKLogin_DO))
        .route("/SDKLogin", any(SDKLogin))
        .route("/isc", any(get_ios_shadowsocks_conf))
        .route("/test", any(NetWorkTest))
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