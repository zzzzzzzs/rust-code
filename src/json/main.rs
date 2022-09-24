#[macro_use]
extern crate json;

fn main() {
    let parsed = json::parse(r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#).unwrap();

    let instantiated = object! {
        "code" => 200,
        "success" => true,
        "payload" => object!{
            "features" => array![
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };

    assert_eq!(parsed, instantiated);
    println!("解析输出字段：code={}", instantiated["code"]);
    println!("解析输出字段：success={}", instantiated["success"]);
    println!("解析输出对象：payload={:?}", instantiated["payload"]);
    println!("解析输出数组：features={:?}", instantiated["payload"]["features"]);
    println!("解析输出数组元素：0={}", instantiated["payload"]["features"][0]);
    println!("解析输出数组元素：1={}", instantiated["payload"]["features"][1]);
    println!("解析输出数组元素：2={}", instantiated["payload"]["features"][2]);
}


// 解析输出字段：code=200
// 解析输出字段：success=true
// 解析输出对象：payload=Object(Object { store: [("features", Array([Short("awesome"), Short("easyAPI"), Short("lowLearningCurve")]), 0, 0)] })
// 解析输出数组：features=Array([Short("awesome"), Short("easyAPI"), Short("lowLearningCurve")])
// 解析输出数组元素：0=awesome
// 解析输出数组元素：1=easyAPI
// 解析输出数组元素：2=lowLearningCurve
