use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;

#[tokio::main]
async fn main() {
    //get请求
    // if let Ok(res)= get_calss_list_out_json().await{
    //     println!("{:#?}",res)
    // };
    //get请求
    if let Ok(res)= get_calss_list_out_text().await{
        println!("{:#?}",res)
    };
    //    post传参加token请求
    //    if let Ok(res)=get_column_list().await{
    //        println!("{:#?}",res)
    //    }
    //post new 对象
    //    if let Ok(res)=get_other().await{
    //     println!("{:#?}",res)
    // }

}

//get请求不含token，输出json 格式
async fn get_calss_list_out_json()->Result<(),Box<dyn std::error::Error>>{
    let res=reqwest::get("http://example.url")
        .await?
        .json::<HashMap<String,Value>>()
        .await?;
    println!("{:#?}",res);
    Ok(())
}
// 输出 text 格式
async fn get_calss_list_out_text()->Result<(),reqwest::Error>{
    let res=reqwest::get("https://curlconverter.com/rust/")
        .await?
        .text()
        .await?;
    println!("aaa {:#?}", res);
    Ok(println!("{:#?}",res))

}

async fn get_column_list()->Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", "Bearer  token_in_here".parse().unwrap());
    // post 参数
    // 组装要提交的数据
    let mut data = HashMap::new();
    data.insert("params", "1");
    Ok(client.post("https://http://example.url").headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
}
async fn get_other() -> Result<(),reqwest::Error> {
    let mut data = HashMap::new();
    data.insert("params1", "1");
    data.insert("params2", "2");
    data.insert("params3", "{\"channel_name\":\"新闻广播\",\"logo\":\"\",\"desc\":\"123\"}");
    data.insert("compereName", "主持人");
    data.insert("status", "1");

    let res = reqwest::Client::new()
        .post("http://example.url")
        .json(&data)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Access-Control-Allow-Origin", "*")
        .send()
        .await?;
    let text = res.text().await?;
    Ok(println!("{:#?}",text))

}
