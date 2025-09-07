// use mysql::*;
// use mysql::prelude::*;
// use serde_json::{json, Value};


// pub fn sql() -> String {
//     let url = "mysql://root:dpoaWmadfLmdLsp@rw.curesky.site:6033/ru_chu_rank";
//     let pool = Pool::new(url).unwrap(); // 获取连接池
//     let mut conn = pool.get_conn().unwrap(); // 获取链接

//     // let mut result = String::new(); // 存储最终返回的字符串

//     // 查询数据库
//     let res: Vec<(String, i32)> = conn.query("SELECT player_name, score FROM player_score").unwrap();
    
//     // 将查询结果格式化为字符串（例如 JSON 或 CSV）
//     let mut output = String::new();
//     for row in res {
//         output.push_str(&format!("{} : {}\n,", row.0, row.1));
//     }
//     if output.is_empty() {
//         "No data found".to_string()
//     } else {
//         output
//     }
// }


// pub fn sql_to_json() -> String {
//     let url = "mysql://root:dpoaWmadfLmdLsp@rw.curesky.site:6033/ru_chu_rank";
    
//     let pool = match Pool::new(url) {
//         Ok(p) => p,
//         Err(e) => return json!({"error": format!("Connection pool failed: {}", e)}).to_string(),
//     };

//     let mut conn = match pool.get_conn() {
//         Ok(c) => c,
//         Err(e) => return json!({"error": format!("Get connection failed: {}", e)}).to_string(),
//     };

//     // 明确指定映射结果的类型为 Value
//     match conn.query_map(
//         "SELECT player_name, score FROM player_score",
//         |(name, score): (String, i32)| -> Value {  // 这里明确指定了类型
//             json!({
//                 "player_name": name,
//                 "score": score
//             })
//         },
//     ) {
//         Ok(results) => {
//             if results.is_empty() {
//                 json!({"status": "success", "message": "No data found", "data": []}).to_string()
//             } else {
//                 json!({
//                     "status": "success",
//                     "count": results.len(),
//                     "data": results
//                 }).to_string()
//             }
//         }
//         Err(e) => json!({
//             "status": "error",
//             "message": format!("Query failed: {}", e)
//         }).to_string(),
//     }
// }
