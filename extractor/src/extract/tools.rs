use scraper::{ElementRef, Selector};
use std::collections::HashMap;

/// ฟังก์ชันสำหรับแปลง Table ที่มีโครงสร้าง <tr><td>Key</td><td>Value</td></tr> ให้เป็น HashMap
pub fn parse_table_to_dict(table_node: ElementRef<'_>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let tr_selector = Selector::parse(":scope > tr, :scope > tbody > tr").unwrap();
    let td_selector = Selector::parse(":scope > td").unwrap();
    let nested_table_selector = Selector::parse("table").unwrap(); // เพิ่มตัวตรวจจับตารางซ้อน

    for row in table_node.select(&tr_selector) {
        // --- จุดที่แก้ไข: เช็คก่อนว่าใน row นี้มี table ซ้อนไหม ---
        // ถ้า row นี้มี table อยู่ข้างใน แสดงว่าเป็น row ของตารางแม่ที่หุ้มตารางลูกไว้
        // เราจะไม่เอามาทำ dict เพราะมันไม่ใช่คู่ Key-Value จริงๆ
        if row.select(&nested_table_selector).next().is_some() {
            continue;
        }

        let cols: Vec<String> = row
            .select(&td_selector)
            .map(|td| {
                td.text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .replace('\n', " ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();

        if cols.len() >= 2 {
            let key = cols[0].replace(':', "").trim().to_string();
            let value = cols[1].clone();

            if !key.is_empty() && key.len() < 50 {
                // กันเหนียว: ถ้า key ยาวเกิน 50 ตัวอักษร ให้เดาว่าไม่ใช่ key จริง
                map.insert(key, value);
            }
        }
    }
    map
}

/// ฟังก์ชันสำหรับดึงข้อมูลจากตารางที่มี column เดียว ให้กลายเป็น List (Vec<String>)
pub fn parse_table_to_list(table_node: ElementRef<'_>) -> Vec<String> {
    let mut list = Vec::new();
    let tr_selector = Selector::parse(":scope > tr, :scope > tbody > tr").unwrap();
    let td_selector = Selector::parse(":scope > td").unwrap();

    for row in table_node.select(&tr_selector) {
        let cols: Vec<ElementRef> = row.select(&td_selector).collect();

        // ตรวจสอบว่ามีช่องเดียว (1 column) ตามที่โจทย์ต้องการ
        if cols.len() == 1 {
            let text_content = cols[0]
                .text()
                .collect::<Vec<_>>()
                .join(" ") // รวม text จากทุก tag ย่อย (b, u, a ฯลฯ)
                .trim()
                .split_whitespace() // ล้างพวกช่องว่างประหลาดๆ
                .collect::<Vec<_>>()
                .join(" ");

            if !text_content.is_empty() {
                list.push(text_content);
            }
        }
    }
    list
}
