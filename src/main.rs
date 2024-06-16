use rand::Rng;
use std::collections::HashSet;

fn main() {
    let cuisines = vec![
        ("川菜", vec!["浪李白", "辛香汇"]),
        ("湘菜", vec!["望师傅", "胡子大厨"]),
        ("粤菜", vec!["文通冰室", "东发道", "荔轼楼", "点都德"]),
        ("鲁菜", vec!["鲁采"]),
    ];
    let mut accept_cuisine: bool = false;
    let mut prompted_cuisine_indexes = HashSet::new();
    let mut selected_cuisine: Option<&(&str, Vec<&str>)> = None;
    while !accept_cuisine {
        let mut random_cuisine_index;
        loop {
            if prompted_cuisine_indexes.len() >= cuisines.len() {
                panic!("你太贪了");
            }
            random_cuisine_index = rand::thread_rng().gen_range(0..cuisines.len());
            if !prompted_cuisine_indexes.contains(&random_cuisine_index) {
                prompted_cuisine_indexes.insert(random_cuisine_index);
                break;
            }
        }
        let prompted_cuisine = &cuisines[random_cuisine_index];
        println!("Random Cuisine: {}", prompted_cuisine.0);
        println!("do you accept this cuisine? (y/n)");

        let mut accept_cuisine_str = String::new();
        std::io::stdin()
            .read_line(&mut accept_cuisine_str)
            .expect("Cannot read whether accept cuisine");
        let trimmed = accept_cuisine_str.trim();
        if trimmed.eq("n") {
            accept_cuisine = false;
        } else if trimmed.eq("y") {
            selected_cuisine = Some(prompted_cuisine);
            accept_cuisine = true;
        } else {
            panic!("you should type y or n!!!");
        }
    }

    let mut accept_restaurant: bool = false;
    let restaurants = &selected_cuisine.unwrap().1;
    let mut prompted_restaurant_indexes = HashSet::new();
    while !accept_restaurant {
        let mut random_restaurant_index;
        loop {
            if prompted_restaurant_indexes.len() >= restaurants.len() {
                panic!("你太贪了");
            }
            random_restaurant_index = rand::thread_rng().gen_range(0..restaurants.len());
            if !prompted_restaurant_indexes.contains(&random_restaurant_index) {
                prompted_restaurant_indexes.insert(random_restaurant_index);
                break;
            }
        }
        let prompted_restaurant = &restaurants[random_restaurant_index];
        println!("Random restaurant: {}", prompted_restaurant);
        println!("do you accept this restaurant? (y/n)");

        let mut accept_restaurant_str = String::new();
        std::io::stdin()
            .read_line(&mut accept_restaurant_str)
            .expect("Cannot read whether accept restaurant");
        let trimmed = accept_restaurant_str.trim();
        if trimmed.eq("n") {
            accept_restaurant = false;
        } else if trimmed.eq("y") {
            accept_restaurant = true;
            println!("Let go to {} today!", prompted_restaurant);
        } else {
            panic!("you should type y or n!!!");
        }
    }
}
