use std::collections::HashMap;

fn main() {
    let mut page_cnts = HashMap::new();

    page_cnts.insert("adv",207);
    page_cnts.insert("gri",751);
    page_cnts.insert("pri",303);


    if !page_cnts.contains_key("les") {
        println!(
            "we know {} books",page_cnts.len()
        );
    }

    for book in ["pri","ali"] {
        match page_cnts.get(book) {
            Some(cnt) => println!("{book}:{cnt} pages"),
            None => println!("{book} is unknown"),
        }
    }

    for book in ["pri","ali"] {
        let page_cnt:&mut i32 = page_cnts.entry(book).or_insert(0);
        *page_cnt+=1; 
    }

    //dbg!(page_cnts);
    let pc1 = page_cnts
    .get("Harry Potter and the Sorcerer's Stone")
    .unwrap_or(&336);
    dbg!(pc1);
    let cnt2 = HashMap::from(
        [
            ("hhh".to_string(),332),
            ("saaa".to_string(),222),
        ]
    );

    dbg!(cnt2);
     
}
