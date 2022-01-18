use chrono::prelude::*;
use rand::Rng;
use reqwest::header::HeaderMap;
use serde_json::json;
use serde_json::value::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process::exit;
use std::process::Command;
use std::path::Path;
mod utils;

pub async fn fanyi() -> Result<(), Box<dyn Error>> {
    
    let url = "https://fanyi.youdao.com/translate_o?smartresult=dict&smartresult=rule";
    // post 请求要创建client
    let client = reqwest::Client::new();
    let (word,dir) = parse_args().unwrap_or_else(|e| {
        println!("{}, process quit", e);
        exit(0)
    });
    prepare(dir).unwrap();
    println!("查询: {}",word);
    let (headers, data) = build_form(word);
    let result = client
        .post(url)
        .headers(headers)
        .form(&data)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    let simple_result = json!(result.get("translateResult"))[0][0]["tgt"]
        .to_string()
        .replace('"', "");
    println!("翻译结果:\n   {}", simple_result);
    println!("更多翻译:");
    let smart_result = result
        .get("smartResult").unwrap_or_else(||{
            println!("    no ruslut");
            exit(0)
        })
        .get("entries")
        .unwrap()
        .as_array()
        .unwrap();
    
    // println!("{:#?}",result);
    for item in smart_result {
        let output = item.to_string().replace("\\r\\n", "").replace("\\n", "").replace('"', "");
        if output == String::from("") {
            continue;
        }
        println!("   {}", output);
    }
    // println!("{:#?}", smartResult);
    // 发起post请求并返回
    Ok(())
}

fn build_form(word: String) -> (HeaderMap, HashMap<String, String>) {
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent","Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36".parse().unwrap());
    headers.insert("Cookie", "OUTFOX_SEARCH_USER_ID=1083764993@10.108.160.100; JSESSIONID=aaa9bLvraL5cie3bdE1Px; OUTFOX_SEARCH_USER_ID_NCOO=1222829925.345313; ___rl__test__cookies=1625468463760".parse().unwrap());
    headers.insert("Referer", "https://fanyi.youdao.com/".parse().unwrap());
    let ts = Local::now().timestamp_millis().to_string();
    let random_number = rand::thread_rng().gen_range(1..10).to_string();
    let salt = format!("{}{}", ts, random_number);
    let sign = format!(
        "{}{}{}{}",
        "fanyideskweb", word, salt, "Y2FYu%TNSbMCxc3t2u^XT"
    );
    let sign_md5 = utils::md5(sign);

    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("i".to_string(), word);
    data.insert("from".to_string(), "AUTO".to_string());
    data.insert("to".to_string(), "AUTO".to_string());
    data.insert("smartresult".to_string(), "dict".to_string());
    data.insert("client".to_string(), "fanyideskweb".to_string());
    data.insert("salt".to_string(), salt.to_string());
    data.insert("sign".to_string(), sign_md5.to_string());
    data.insert("lts".to_string(), ts.to_string());
    data.insert(
        "bv".to_string(),
        "5b3e307b66a6c075d525ed231dcc8dcd".to_string(),
    );
    data.insert("doctype".to_string(), "json".to_string());
    data.insert("version".to_string(), "2.1".to_string());
    data.insert("keyfrom".to_string(), "fanyi.web".to_string());
    data.insert("action".to_string(), "FY_BY_REALTlME".to_string());
    (headers, data)
}

fn parse_args() -> Result<(String,String),&'static str>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("invaild arguments");
    }
    Ok((args[1].clone(),args[0].clone()))
}

fn prepare(_dir:String)->Result<(),Box<dyn Error>>{
    if !fs::metadata(Path::new("/usr/bin/youdao")).is_ok(){
        let cmd = format!("sudo ln -s $(pwd)/{} /usr/bin/youdao",_dir.replace("./", "/"));
        Command::new("sh").arg("-c").arg(cmd).output().expect("exec sh failed");
    }
    Ok(())
}

