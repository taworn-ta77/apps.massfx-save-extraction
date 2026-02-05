use crate::extract::tools::{parse_table_to_dict, parse_table_to_list};
use scraper::{ElementRef, Html, Selector};

#[derive(Default, Debug, Clone)]
struct Scraper {
    pub url: String,
    pub paragon: String,
    pub name: String,
}

async fn fetch_each_save(save_node: ElementRef<'_>) -> Result<(), Box<dyn std::error::Error>> {
    //println!("oooooooooo");
    //println!("- {}\n", save_node.value().name());

    let selector = Selector::parse("tr > td > table").unwrap();
    let inner = save_node.select(&selector).next().unwrap();

    let _map = parse_table_to_dict(inner);
    //for (k, v) in map.iter() {
    //println!("{}: {}", k, v);
    //}
    //println!("dict count: {}\n", map.len());

    let list = parse_table_to_list(inner);
    for _i in list.iter() {
        //println!("{}", i);
    }
    //println!("list count:{}\n", list.len());

    let selector = Selector::parse(":scope > tbody > tr > td:nth-child(2)").unwrap();
    let outer = save_node.select(&selector).next().unwrap();
    let _description = outer
        .text()
        .map(|t| t.trim()) // ตัด space หัวท้ายของแต่ละก้อน
        //.filter(|t| !t.is_empty()) // กรองก้อนว่างๆ ออก
        .collect::<Vec<_>>()
        .join("\n");
    //println!("----------");
    //println!("{}", description);

    //println!("{}\n\n", save_node.text().collect::<String>());
    Ok(())
}

async fn fetch_url(s: &Scraper) -> Result<(), Box<dyn std::error::Error>> {
    println!("fetching {}, {}, {}", s.name, s.paragon, s.url);

    let res = reqwest::get(&s.url).await?;
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
    let list: Vec<Scraper> = vec![
        // Male Paragon
        Scraper {
            url: String::from("https://masseffectsaves.com/mpsa.php"),
            paragon: String::from("Paragon"),
            name: String::from("Ashley"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/mpsl.php"),
            paragon: String::from("Paragon"),
            name: String::from("Liara"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/mpsn.php"),
            paragon: String::from("Paragon"),
            name: String::from("No one"),
        },
        // Male Renegade
        Scraper {
            url: String::from("https://masseffectsaves.com/mrsa.php"),
            paragon: String::from("Renegade"),
            name: String::from("Ashley"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/mrsl.php"),
            paragon: String::from("Renegade"),
            name: String::from("Liara"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/mrsn.php"),
            paragon: String::from("Renegade"),
            name: String::from("No one"),
        },
        // Male Mix
        Scraper {
            url: String::from("https://masseffectsaves.com/mmsa.php"),
            paragon: String::from("Mixed Alignment"),
            name: String::from("Ashley"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/mmsl.php"),
            paragon: String::from("Mixed Alignment"),
            name: String::from("Liara"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/mmsn.php"),
            paragon: String::from("Mixed Alignment"),
            name: String::from("No one"),
        },
        // Female Paragon
        Scraper {
            url: String::from("https://masseffectsaves.com/fpsk.php"),
            paragon: String::from("Paragon"),
            name: String::from("Kaidan"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/fpsl.php"),
            paragon: String::from("Paragon"),
            name: String::from("Liara"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/fpsn.php"),
            paragon: String::from("Paragon"),
            name: String::from("No one"),
        },
        // Female Renegade
        Scraper {
            url: String::from("https://masseffectsaves.com/frsk.php"),
            paragon: String::from("Renegade"),
            name: String::from("Kaidan"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/frsl.php"),
            paragon: String::from("Renegade"),
            name: String::from("Liara"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/frsn.php"),
            paragon: String::from("Renegade"),
            name: String::from("No one"),
        },
        // Female Mix
        Scraper {
            url: String::from("https://masseffectsaves.com/fmsk.php"),
            paragon: String::from("Mixed Alignment"),
            name: String::from("Kaidan"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/fmsl.php"),
            paragon: String::from("Mixed Alignment"),
            name: String::from("Liara"),
        },
        Scraper {
            url: String::from("https://masseffectsaves.com/fmsn.php"),
            paragon: String::from("Mixed Alignment"),
            name: String::from("No one"),
        },
    ];

    for i in list.iter() {
        if let Err(e) = fetch_url(&i).await {
            eprintln!("{}", e.to_string())
        }
    }

    ()
}
