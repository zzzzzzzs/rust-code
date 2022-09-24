use uuid::Uuid;
use chrono::prelude::*;
use simple_excel_writer::*;


pub fn export() -> String {
    //本地存储路径
    let save_path = "C:/";
    //文件代理路径
    let url_path = "http://127.0.0.1/";
    // 当前时间
    let date = Local::now().format("%Y-%m-%d").to_string();
    std::fs::create_dir(format!("{}{}/", save_path, date.clone()));
    let uid = Uuid::new_v4().to_string()[0..8].to_string();
    let filename = format!("{}{}/{}.xlsx", save_path, date, uid);
    let url_name = format!("{}{}/{}.xlsx", url_path, date, uid);
    let mut wb = Workbook::create(&filename);
    let mut sheet = wb.create_sheet("第一页");
    // 设置行宽
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });

    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["姓名", "标题"])?;
        sw.append_row(row!["张三","你好"])
    }).expect("写入excel错误!");

    let mut sheet = wb.create_sheet("第二页");
    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["姓名", "标题"])?;
        sw.append_row(row!["张三","你好"])
    }).expect("写入excel错误!");
    wb.close().expect("关闭excel错误!");
    url_name
}


fn main() {
    let url = export();
    println!("{}", url);

}