
use std::{env, default, any::TypeId};

// use diesel::*;
use mysql::{Pool,  QueryResult, prelude::Queryable, Params, Row, params, PooledConn};
use serde::*;

pub fn getconn(url:String)->Pool{

    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(&url).unwrap());

    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default())).unwrap();
    // let pool=PgConnection::establish(&url)
    // .unwrap_or_else(|_| panic!("Error connecting to {}", url));

    
    pool
}
pub fn pscalewrite()->Pool{
    let url = env::var("DATAW").unwrap();
    getconn(url)
}
pub fn pscaleread()->Pool{
    let url = env::var("DATAR").unwrap();
    getconn(url)
}
// pub fn addtosessiondb(datatoadd:Vec<sessioncount>){
//     // createtable(&pscalewrite());
//     // println!("Successfully connected to Write to PlanetScale!");
//     insertintoscdb(&pscalewrite(), &datatoadd);
        
   
//     // println!("Successfully connected to Read from PlanetScale!");
//     // printdata(&pscaleread());

// }
// pub fn addtoosdb(datetofetch:&str,datatoadd:osl){
//     // createtable(&pscalewrite());
//     // println!("Successfully connected to Write to PlanetScale!");
//     insertintoosdb(&pscalewrite(), datetofetch,&datatoadd);
        
   
//     // println!("Successfully connected to Read from PlanetScale!");
//     // printdata(&pscaleread());

// }
// pub fn addtoeventdb(datetofetch:&str,datatoadd:(Vec<eventcount>,i32)){
//     // createtable(&pscalewrite());
//     // println!("Successfully connected to Write to PlanetScale!");
//     insertintoeventdb(&pscalewrite(),datetofetch, &datatoadd);
        
   
//     // println!("Successfully connected to Read from PlanetScale!");
//     // printdata(&pscaleread());

// }
// pub fn createtable(pool:&Pool){
//     let mut conn = pool.get_conn().unwrap();
//     let createtable=format!(
//         "CREATE TABLE ac_oses (
//         date VARCHAR(50) PRIMARY KEY,
//         os_name VARCHAR(50) NOT NULL,
//         count INT NOT NULL
//       );
//       ");
//     let mut saved=false;
//     if let Ok(res) = conn.exec_drop(
//         createtable,{}
//     ) {
//         // let vc:Vec<(String,i32)>=res;
//         println!("added");
//         saved=true;
//     }
//     if !saved {

//         println!("gone through");
//     }
    
// }
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct eachuser{
    pub id:String,
    pub url:String,
    // pub uid:String,
    // pub pswd:String
    
}
fn parse_row_as_data(mut row: mysql::Row) -> eachuser {
    let mut bill = eachuser::default();

    bill.id = row.take("id").unwrap();
    // bill.url = row.take("uid").unwrap();
    bill.url = row.take("url").unwrap();
    // bill.pswd = row.take("pswd").unwrap();

    bill
    // ...
}
pub fn printdata()-> Result<String,()>{
    let pool=pscaleread();
    let mut _conn = pool.get_conn().unwrap();
    let mut results:Vec<Row> = _conn .query("SELECT * from urls").unwrap();
    let mut svec=String::new();
    for eacha in &results{

        svec.push_str(&format!("{:?}",parse_row_as_data(eacha.clone())));
    }
    Ok(svec)
}
// fn addeachtoscdb(mut conn:&mut PooledConn)->Result<(),()>{
//     let mut saved=false;
//     // let id=TypeId::of::<T>();
//     // let idofsc=TypeId::of::<sessioncount>() ;
//     //     let commandtoexec=match (id) {
//     //         idofsc=>{
//     //             conn.exec(
//     //                 r"INSERT INTO ac_events (date, count)
//     //             VALUES (?, ?)
//     //             ON DUPLICATE KEY UPDATE count = IF(VALUES(count) > ac_events.count, VALUES(count), ac_events.count);",
//     //             (esc.datetime.clone(), esc.count.clone()))
//     //         },
//     //         _=>{
//     //             conn.exec(
//     //                 r"INSERT INTO ac_events (date, count)
//     //             VALUES (?, ?)
//     //             ON DUPLICATE KEY UPDATE count = IF(VALUES(count) > ac_events.count, VALUES(count), ac_events.count);",
//     //             (esc.datetime.clone(), esc.count.clone()))
//     //         }
//     //     };

//     if let Ok(res) = conn.exec(
//             r"INSERT INTO urls (date, count)
//         VALUES (?, UNHEX(MD5(?)))
//         ON DUPLICATE KEY UPDATE count = IF(VALUES(count) > ac_events.count, VALUES(count), ac_events.count);",
//         (esc.datetime.clone(), esc.count.clone())
//     ) {
//         let vc:Vec<(String,i32)>=res;
//         println!("new record");
//         saved=true;
//     }
//     if !saved {

//         println!("gone through");
//     }
//     Ok(())
// }
// fn addeachtoosdb(mut conn:&mut PooledConn,df:&str,esc:&osl)->Result<(),()>{
//     let mut saved=false;
//     // let id=TypeId::of::<T>();
//     // let idofsc=TypeId::of::<sessioncount>() ;
//     //     let commandtoexec=match (id) {
//     //         idofsc=>{
//     //             conn.exec(
//     //                 r"INSERT INTO ac_events (date, count)
//     //             VALUES (?, ?)
//     //             ON DUPLICATE KEY UPDATE count = IF(VALUES(count) > ac_events.count, VALUES(count), ac_events.count);",
//     //             (esc.datetime.clone(), esc.count.clone()))
//     //         },
//     //         _=>{
//     //             conn.exec(
//     //                 r"INSERT INTO ac_events (date, count)
//     //             VALUES (?, ?)
//     //             ON DUPLICATE KEY UPDATE count = IF(VALUES(count) > ac_events.count, VALUES(count), ac_events.count);",
//     //             (esc.datetime.clone(), esc.count.clone()))
//     //         }
//     //     };

//     if let Ok(res) = conn.exec(
//             r"INSERT INTO ac_oses (date, os_name, count)
//         VALUES (?, ?, ?)
//         ON DUPLICATE KEY UPDATE count = IF(VALUES(count) > ac_oses.count, VALUES(count), ac_oses.count);",
//         (df.clone(),&serde_json::to_string(&esc.oses).unwrap(), esc.total.clone())
//     ) {
//         let vc:Vec<(String,i32)>=res;
//         // println!("new record added:{:?}",vc);
//         println!("new record added.");
//         saved=true;
//     }
//     if !saved {

//         println!("gone through");
//     }
//     Ok(())
// }
// fn addeachtoecdb(mut conn:&mut PooledConn,df:&str,esc:&(Vec<eventcount>,i32))->Result<(),()>{
//     let mut saved=false;

//     if let Ok(res) = conn.exec(
//             r"INSERT INTO ac_eventlist (date, eventslist, count)
//         VALUES (?, ?, ?)
//         ON DUPLICATE KEY UPDATE count = IF(VALUES(count) > ac_eventlist.count, VALUES(count), ac_eventlist.count);",
//         (df.clone(),serde_json::to_string(&esc.0).unwrap(), esc.1.clone())
//     ) {
//         let vc:Vec<(String,i32)>=res;
//         println!("new record");
//         saved=true;
//     }
//     if !saved {

//         println!("gone through");
//     }
//     Ok(())
// }
// pub fn insertintoscdb(pool: &Pool, sc:&Vec<sessioncount>) {
//     // let payments = vec![
//     //     sessioncount { datetime: "2023-05-07".to_string(), count: 0 },
//     // ];
//     let mut conn = pool.get_conn().unwrap();
//     for esc in sc{
//         if(esc.count>0){

//             addeachtoscdb(&mut conn,esc);
//         }
//     }
//     // conn.exec_batch(
//     //     r"INSERT INTO ac_events (date,count)
//     //       VALUES (:date, :count)",
//     //     sc.iter().map(|p| params! {
//     //         "date" => p.datetime.clone(),
//     //         "count" => p.count,
//     //     })
//     // ).unwrap();
    
//     // Ok(results)
// }
// pub fn insertintoeventdb(pool: &Pool,df:&str, sc:&(Vec<eventcount>,i32)) {
//     // let payments = vec![
//     //     sessioncount { datetime: "2023-05-07".to_string(), count: 0 },
//     // ];
//     let mut conn = pool.get_conn().unwrap();
//     // for esc in sc{
//         if(sc.1>0){

//             addeachtoecdb(&mut conn,df,sc);
//         }
//     // }
//     // conn.exec_batch(
//     //     r"INSERT INTO ac_events (date,count)
//     //       VALUES (:date, :count)",
//     //     sc.iter().map(|p| params! {
//     //         "date" => p.datetime.clone(),
//     //         "count" => p.count,
//     //     })
//     // ).unwrap();
    
//     // Ok(results)
// }
// pub fn insertintoosdb(pool: &Pool,df:&str, sc:&osl) {
//     // let payments = vec![
//     //     sessioncount { datetime: "2023-05-07".to_string(), count: 0 },
//     // ];
//     let mut conn = pool.get_conn().unwrap();
//     // for esc in sc{
//         if(sc.total>0){

//             addeachtoosdb(&mut conn,&df,sc);
//         }
//     // }
//     // conn.exec_batch(
//     //     r"INSERT INTO ac_events (date,count)
//     //       VALUES (:date, :count)",
//     //     sc.iter().map(|p| params! {
//     //         "date" => p.datetime.clone(),
//     //         "count" => p.count,
//     //     })
//     // ).unwrap();
    
//     // Ok(results)
// }