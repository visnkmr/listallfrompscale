// use std::{env, any::TypeId};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
// use commitstojson::commitstojson;
// use pscale::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{blocking::Client, header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION}};
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::{Value, json};
use dotenv::dotenv;
// mod pscale;
// #[test]
//the codeberg and gitea server stats getting api

use rand::seq::SliceRandom;

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
#[test]
fn trydbcon(){
    dotenv().ok();
    // let data=printeuser("ram".to_string(),String::new()).unwrap().url;
    // let jdata:Vec<String>=serde_json::from_str(&data).unwrap();
    // println!("{:?}",jdata);
    // println!("{:?}",data);
    // adddatatouser("ram".to_string(), "google.com".to_string());
    println!("{:?}",printdata());
}
pub fn printdata()-> Result<String,()>{
    let pool=pscaleread();
    let salt = env::var("SALT").unwrap();

    let mut _conn = pool.get_conn().unwrap();
    let mut results:Vec<Row> = _conn .query(format!("SELECT * from urls")).unwrap();
    let mut svec=String::new();
    for eacha in &results{

        svec.push_str(&format!("{:?}",parse_row_as_data(eacha.clone())));
    }
    Ok(svec)
}
pub fn printeuser(uid:String,pswd:String)-> Result<eachuser,()>{
    let pool=pscaleread();
    let salt = env::var("SALT").unwrap();

    let mut _conn = pool.get_conn().unwrap();
    let mut results:Vec<Row> = _conn .query(format!("SELECT * from urls WHERE uid=UNHEX(MD5('{}{}'))",uid,salt)).unwrap();
    
    Ok(parse_row_as_data(results.get(0).unwrap().clone()))
}
pub fn adddatatouser(uid:String,datatoadd:String)-> Result<eachuser,()>{
    let pool=pscalewrite();
    let salt = env::var("SALT").unwrap();

    let mut _conn = pool.get_conn().unwrap();
    let mut results:Vec<Row> = _conn .query(format!("UPDATE urls SET url = JSON_ARRAY_APPEND(url, '$', '{}') WHERE uid=UNHEX(MD5('{}{}'));",datatoadd,uid,salt)).unwrap();
    
    Ok(parse_row_as_data(results.get(0).unwrap().clone()))
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

pub fn choose_starter() -> String {
    let pokemons = vec!["Bulbasaur", "Charmander", "Squirtle", "Pikachu"];
    let starter = pokemons.choose(&mut rand::thread_rng()).unwrap();
    starter.to_string()
}

// #[tokio::main]
//  pub async fn main()-> Result<(), Box<dyn std::error::Error>>{
 pub fn testdata()-> Result<(), Box<dyn std::error::Error>>{

    dotenv().ok();
    // println!("{:?}",printdata());
    // commitstojson();
    // let today = Utc::now();
    // let date_28_days_ago = &(today - chrono::Duration::days(27)).format("%Y-%m-%d").to_string();
    // let date_yesterday = &(today - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    // let date_today = &(today ).format("%Y-%m-%d").to_string();
    // // commitstojson::commitstojson();
    // // println!("{:?}",TypeId::of::<sessioncount>());

    // // //add commits to json.
    // // commitstojson();

    // // adding session count per day from appcenter to planetscale.
    // let vecssc:Vec<sessioncount>=appcentervecapi("session_counts",&date_28_days_ago,&date_yesterday).await?;
    // addtosessiondb(vecssc);

    // for i in 1..27{  
    //     let datetofetch=&(today - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
    //     let (vecsevents)=eventsapi("events",&datetofetch,&datetofetch).await?;
    //     // println!("{:?}---{}",serde_json::to_string(&vecsevents).unwrap(),serde_json::to_string(&vecsevents).unwrap().len());
    //     addtoeventdb(&datetofetch,vecsevents);
    // }

    // //adding os versions per day from appcenter to planetscale.
    // for i in 1..27{
    //     println!("checking {} day before",i);
    //     let datetofetch=&(today - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
    //     let vecstoadd=osapi("oses",&datetofetch,&datetofetch).await?;
    //     // println!("{}",serde_json::to_string(&vecstoadd.oses).unwrap().len());
    //     addtoosdb(datetofetch,vecstoadd);
    // }
    // println!("{:?}",vecstoadd);
    
    Ok(())
    // Ok("Yes".to_string())
}




//gitea codeberg commit get api and sort by timestamp




fn print_key_value_pairs(value: &Value) {
    if let Some(object) = value.as_object() {
        for (key, value) in object.iter() {
            println!("{}: {}", key, value);
        }
    }
}
#[test]
fn datetest(){
    let g=NaiveDateTime::parse_from_str("2023-05-12T15:01:34+05:30","%Y-%m-%dT%H:%M:%S%z")
                                    .unwrap();
                                
    // let g1=DateTime::parse_from_str("2022-12-06T18:31:45","%Y-%m-%dT%H:%M:%S")
    //                                 .unwrap();

    let ndt = NaiveDateTime::parse_from_str("2022-12-06T18:31:45Z", "%Y-%m-%dT%H:%M:%SZ").unwrap();

                                    // .with_timezone(&FixedOffset::east_opt(5*3600+30*60).unwrap());
}

