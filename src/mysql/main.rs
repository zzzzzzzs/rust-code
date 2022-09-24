use mysql::prelude::*;
use mysql::*;

fn main() {
    //设置连接字符串
    let url = "mysql://root:111@localhost:3366/test";
    let opts = Opts::from_url(url).unwrap();// 类型转换将 url 转为opts
    //连接数据库 这里 老版本是直接传url 字符串即可 新版本21版要求必须为opts类型
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //数据库操作

    //流式查询  数据逐行读取，数据不会存储在内存中
    conn.query_iter("select * from dim_user")
        .unwrap()
        .for_each(|row| {
            let r: (String, String) = from_row(row.unwrap());
            println!("{},{}", r.0, r.1);
        });
}