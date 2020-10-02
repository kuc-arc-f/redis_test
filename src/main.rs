// Rust + redis, 書込み。シリアライズ、List.rpush
//


extern crate redis;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use redis::{Commands};
use serde::{Deserialize, Serialize};

//
pub fn get_content(filename: String ) -> String{
//    println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

//    println!("With text:\n{}", contents);
    return contents;
}
//
#[derive(Serialize, Deserialize , Debug)]
struct TaskItem {
    id: i64,
    title: String,
    content: String,
} 
//
fn test2(items: Vec<TaskItem>) -> redis::RedisResult<()>{
    let client = redis::Client::open("redis://localhost/").expect("url error");
    let mut connection = client.get_connection().expect("connect error");

    let key3 = "list_3";  
    for row in &items {
        let s_title = &row.title;
        let s_content = &row.content;  
        let serialized = serde_json::to_string(&row ).unwrap();
//        println!( "{}", &s_title );
        let result2: u8 = connection.rpush(key3, &serialized ).unwrap();        
    }

    Ok(())
}

//
fn main() {
    println!("#start");
    let fname = "/home/naka/work/node/express/app7/public/tasks.json";
    let json = get_content( fname.to_string() );
    let deserialized: Vec<TaskItem> = serde_json::from_str(&json).unwrap();

    let r =test2(deserialized);
}
