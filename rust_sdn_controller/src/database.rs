use std::collections::HashMap;
use std::result::Result;
use mysql::*;
use mysql::prelude::*;

pub fn get_policy() -> Result<HashMap<u32, u32>, Box<dyn std::error::Error>>{
    let url = "mysql://peace:gy6vv5JqWeHiJ9kfoN0zJOUW8xe@localhost:3306/peace_zorigtbaatar";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    
    let selected_policy = conn
        .query_map(
            "SELECT src_ip, dst_ip from rust_policy",
            |(src_ip, dst_ip): (u32, u32)| {
                [src_ip, dst_ip]
            },
        )?;

    let mut policy = HashMap::new();
    for p in &selected_policy {
        policy.insert(p[0], p[1]);
    }
    let answer = Ok(policy);
    answer
}