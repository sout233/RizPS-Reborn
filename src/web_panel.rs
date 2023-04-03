use std::collections::HashMap;
use axum::{
    routing::any,
    http::{header::{HeaderMap, HeaderName, HeaderValue},header},
    response::Json,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use colored::Colorize;
use tokio::fs;

static mut ALLOW_TOKENS: Vec<String> = vec![];

pub async fn start_webpanel(listen_ip: String,listen_port: String) {
    println!("{} -> WebUi已启动 可在https://{}:{}上访问", "WebUi".green(), listen_ip, listen_port);

    let app = Router::new()
        .route("/auth",any(auth))
        .route("/panel/:token",any(get_panel_html))
        .route("/panel/js/:token",any(get_panel_js))
        .route("/indexjs.js", any(get_root_js))
        .route("/shutdown_rizps/:token",any(shutdownrzpr))
        .route("/get_username_list/:token",any(get_username_list))
        .route("/aclist/:token",any(get_aclist_html))
        .route("/aclist/js/:token",any(get_aclist_js))
        .route("/ac_deatil/:username/:token",any(get_user_deatil))
        .route("/", any(get_root));

    let tls_config = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem"
    ).await.unwrap();

    axum_server::bind_rustls((listen_ip + ":" + &*listen_port).parse().unwrap(), tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user_deatil(axum::extract::Path(down_url): axum::extract::Path<HashMap<String,String>>) -> (HeaderMap,String){
    unsafe {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/html")
        );
        if(ALLOW_TOKENS.contains(down_url.get("token").unwrap())){
            let ac = crate::get_user_account(crate::get_serde_accountfile(),down_url.get("username").unwrap().to_owned());
            let mut ret: String = ("<head><meta charset=\"utf-8\"><title>Account Deatil - RizPS-Reborn Control Panel</title></head><h1>账号信息</h1><h3>用户名：</h3>".to_string() + &*ac.sdklogin_username + &*"<h3>用户游戏内名称：</h3>".to_string() + &*ac.sdklogin_gamename + &*"<h3>用户Coin数量：</h3>".to_string() + &*ac.sdklogin_coin.to_string() + &*"<h3>用户dot数量：</h3>".to_string() + &*ac.sdklogin_dot.to_string() + &*"<h3>用户已解锁关卡：</h3>".to_string()).to_string();
            for i in ac.sdklogin_uklevels{
                ret = ret + &*i.to_string() + "<br/>";
            }
            (headers,ret)
        }
        else{
            (headers,"Token ERROR".to_string())
        }
    }
}

async fn get_username_list(axum::extract::Path(down_url): axum::extract::Path<HashMap<String,String>>) -> String{
    unsafe {
        if(ALLOW_TOKENS.contains(down_url.get("token").unwrap())){
            let ac_serde = crate::get_serde_accountfile();
            let mut ret = "".to_string();
            for i in ac_serde.rzprac_items {
                ret = ret + &*i.sdklogin_username + ">^<";
            }
            ret
        }
        else{
            "Token ERROR".to_string()
        }
    }
}

async fn get_root() -> (HeaderMap, String){
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/html")
    );
    (headers,std::fs::read_to_string("www/index.html").unwrap())
}

async fn shutdownrzpr(axum::extract::Path(down_url): axum::extract::Path<HashMap<String,String>>) -> String{
    unsafe {
        if(ALLOW_TOKENS.contains(down_url.get("token").unwrap())){
            std::process::exit(0);
            "OK".to_string()
        }
        else{
            "Token ERROR".to_string()
        }
    }
}

async fn get_root_js() -> (HeaderMap, String){
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/javascript")
    );
    (headers,std::fs::read_to_string("www/indexjs.js").unwrap())
}

async fn get_aclist_js(axum::extract::Path(down_url): axum::extract::Path<HashMap<String,String>>) -> (HeaderMap, String){
    unsafe {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/javascript")
        );
        if (ALLOW_TOKENS.contains(down_url.get("token").unwrap())) {
            return (headers,std::fs::read_to_string("www/accountlistjs.js").unwrap())
        } else {
            (headers,"Token ERROR".to_string())
        }
    }
}

async fn get_aclist_html(axum::extract::Path(down_url): axum::extract::Path<HashMap<String,String>>) -> (HeaderMap, String){
    unsafe {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/html")
        );
        if (ALLOW_TOKENS.contains(down_url.get("token").unwrap())) {
            return (headers,std::fs::read_to_string("www/accountlist.html").unwrap())
        } else {
            (headers,"Token ERROR".to_string())
        }
    }
}

async fn get_panel_html(axum::extract::Path(down_url): axum::extract::Path<HashMap<String,String>>) -> (HeaderMap, String){
    unsafe {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/html")
        );
        if (ALLOW_TOKENS.contains(down_url.get("token").unwrap())) {
            return (headers,std::fs::read_to_string("www/panel.html").unwrap())
        } else {
            (headers,"Token ERROR".to_string())
        }
    }
}

async fn get_panel_js(axum::extract::Path(down_url): axum::extract::Path<HashMap<String,String>>) -> (HeaderMap, String){
    unsafe {
        let mut headers = HeaderMap::new();
        if (ALLOW_TOKENS.contains(down_url.get("token").unwrap())) {
            return (headers,std::fs::read_to_string("www/panelscripts.js").unwrap())
        } else {
            (headers,"Token ERROR".to_string())
        }
    }
}

async fn auth(post_pwd: String) -> String{
    println!("{} -> 正在尝试Auth，密码：{}","WebUi".green(),post_pwd);
    let server_conf_file = std::fs::File::open("./config.json").unwrap();
    let server_conf: serde_json::Value = serde_json::from_reader(server_conf_file).unwrap();
    if(server_conf["webpanel"]["webpanel_password"].to_string().replace("\"", "") == post_pwd){
        let new_token = crate::generate_random_string(18);//给个token
        unsafe { ALLOW_TOKENS.push(new_token.clone()); }
        println!("{} -> Auth通过，新Token已push进ALLOW_TOKENS","WebUi".green());
        new_token
    }
    else{
        "pwd_err".to_string()
    }
}