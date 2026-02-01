use crate::extract::tools::{parse_table_to_dict, parse_table_to_list};
use scraper::{ElementRef, Html, Selector};

async fn fetch_each_save(save_node: ElementRef<'_>) -> Result<(), Box<dyn std::error::Error>> {
    println!("oooooooooo");
    println!("{}\n", save_node.value().name());

    let selector = Selector::parse("tr > td > table").unwrap();
    let inner = save_node.select(&selector).next().unwrap();

    let map = parse_table_to_dict(inner);
    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }
    println!("dict count: {}\n", map.len());

    let list = parse_table_to_list(inner);
    for i in list.iter() {
        println!("{}", i);
    }
    println!("list count:{}\n", list.len());

    let selector = Selector::parse(":scope > tbody > tr > td:nth-child(2)").unwrap();
    let outer = save_node.select(&selector).next().unwrap();
    let description = outer
        .text()
        .map(|t| t.trim()) // ตัด space หัวท้ายของแต่ละก้อน
        //.filter(|t| !t.is_empty()) // กรองก้อนว่างๆ ออก
        .collect::<Vec<_>>()
        .join("\n");
    println!("----------");
    println!("{}", description);

    //println!("{}\n\n", save_node.text().collect::<String>());
    Ok(())
}

async fn fetch_paragon_ashley() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://masseffectsaves.com/mpsa.php";
    println!("fetching: {}", url);

    let res = reqwest::get(url).await?;
    if !res.status().is_success() {
        return Err(format!("Failed to fetch URL: {}", res.status()).into());
    }

    let body = res.text().await?;
    let html = Html::parse_document(&body);
    let selector = Selector::parse("#mainarea > table > tbody > tr > td > table").unwrap();

    let mut count = 0;
    for element in html.select(&selector) {
        fetch_each_save(element).await?;
        count += 1;
    }

    if count <= 0 {
        println!("No nodes found!");
    } else {
        println!("{} saves found.", count);
    }

    Ok(())
}

pub async fn fetch_save1() -> () {
    let url = "https://masseffectsaves.com";
    println!("fetching: {}", url);

    if let Err(e) = fetch_paragon_ashley().await {
        eprintln!("{}", e.to_string())
    }

    ()
}
