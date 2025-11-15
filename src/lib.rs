// use std::{env, any::TypeId};
use dotenv::dotenv;
use mysql::{prelude::Queryable, Pool, Row};
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::env;

pub fn getconn(url:String)->Pool{
    // let mut ssl_opts = SslOpts::default().with_danger_accept_invalid_certs(true);
    // let ca_cert=env::var("CA_CERT").unwrap();
    // let dec:String=serde_json::from_str(&ca_cert).unwrap();
    // ssl_opts = ssl_opts.with_root_cert_path(Some((&dec.clone())));
    
    let builder = mysql::OptsBuilder::from_opts(match mysql::Opts::from_url(&url) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error parsing database URL: {}", e);
            return mysql::Pool::new(mysql::OptsBuilder::default()).unwrap();
        }
    });

    let pool = match mysql::Pool::new(builder) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error creating database pool: {}", e);
            return mysql::Pool::new(mysql::OptsBuilder::default()).unwrap();
        }
    };
    // let pool=PgConnection::establish(&url)
    // .unwrap_or_else(|_| panic!("Error connecting to {}", url));

    
    pool
}
pub fn pscalewrite()->Pool{
    let url = match env::var("DATAW") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error reading DATAW environment variable: {}", e);
            return mysql::Pool::new(mysql::OptsBuilder::default()).unwrap();
        }
    };
    getconn(url)
}
pub fn pscaleread()->Pool{
    let url = match env::var("DATAR") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error reading DATAR environment variable: {}", e);
            return mysql::Pool::new(mysql::OptsBuilder::default()).unwrap();
        }
    };
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
pub fn createtable(){
    let pool=pscalewrite();
    
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return;
        }
    };
    let createurltable=format!(
        "
        CREATE TABLE `urls` (
            `id` binary(16) NOT NULL,
            `url` json NOT NULL,
            PRIMARY KEY (`id`)
          );
      ");
      
    //   let createredistable=format!(
    //     "
    //     CREATE TABLE `redis` (
    //         `id` char(36) NOT NULL,
    //         `value` json NOT NULL,
    //         PRIMARY KEY (`id`)
    //     );
    //   ");
    let mut saved=false;
    if let Ok(_res) = conn.exec_drop(
        createurltable,{}
    ) {
        // let vc:Vec<(String,i32)>=res;
        println!("added");
        saved=true;
    }
    if !saved {

        println!("gone through");
    }
    
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
// #[derive(QueryableByName,Serialize, Deserialize, Default, Debug, Clone)]
pub struct Eachuser{
    // #[sql_type = "Text"]
    pub id:String,
    // pub id:Vec<u8>,
    // #[sql_type = "Text"]
    pub url:String,
    // pub uid:String,
    // pub pswd:String
    
}
fn parse_row_as_data(uid:String,mut row: mysql::Row) -> Eachuser {
    let mut bill = Eachuser::default();

    // Get the binary ID and convert it to hex string for display
    bill.id = match row.take::<Vec<u8>, _>("uid") {
        Some(binary_id) => {
            // Convert binary to hex string
            let hex_string: String = binary_id.iter().map(|byte| format!("{:02x}", byte)).collect();
            println!("Decoded UID (hex): {}", hex_string);
            hex_string
        },
        None => {
            eprintln!("Error taking id from row");
            String::new()
        }
    };
    
    bill.url = match row.take("url") {
        Some(url) => url,
        None => {
            eprintln!("Error taking url from row");
            String::new()
        }
    };
    // bill.pswd = row.take("pswd").unwrap();

    bill
    // ...
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Eachredisentry{
    // pub id:String,
    pub value:String,
    // pub uid:String,
    // pub pswd:String
    
}
fn parse_value_from_data(mut row: mysql::Row) -> Eachredisentry {
    let mut bill = Eachredisentry::default();

    // bill.url = row.take("uid").unwrap();
    bill.value = match row.take("value") {
        Some(value) => value,
        None => {
            eprintln!("Error taking value from row");
            String::new()
        }
    };
    // bill.pswd = row.take("pswd").unwrap();

    bill
    // ...
}
#[test]
fn trydbcon(){
    dotenv().ok();
    createtable();
//     let file_contents = fs::read_to_string("./ca.pem")
//     .expect("Should have been able to read the file");
// let file_contents=serde_json::to_string(&file_contents).unwrap();
//     println!("{}",file_contents);
    // let dec:String=serde_json::from_str(&file_contents).unwrap();
    // println!("{}",dec);
    // createtable();
    // let ab=Path::new("./ca.pem");
    // if(ab.exists()){
        // // let uname="vis".to_string();
        // // let pwd="example".to_string();
        //                 let uname="345".to_string();
        //                 // // // let data=printeuser("ram".to_string(),String::new()).unwrap().url;
        //                 // // // let jdata:Vec<String>=serde_json::from_str(&data).unwrap();
        //                 // // // println!("{:?}",jdata);
        //                 // // // println!("{:?}",data);
        //                 // // // println!("{:?}",createuser("vis".to_string(), "example".to_string()));
        //                 // // // println!("{:?}",deleteuser("meg".to_string(), "example".to_string()));
        //                 let sdp_offer = json!({
        //                     "type": "offer",
        //                     "sdp": "v=0\r\no=- 8748985181318156403 2 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\na=group:BUNDLE 0\r\na=extmap-allow-mixed\r\na=msid-semantic: WMS\r\nm=application 43271 UDP/DTLS/SCTP webrtc-datachannel\r\nc=IN IP4 117.207.43.69\r\na=candidate:1766165686 1 udp 2113937151 793f23b1-475f-4f64-b97e-a44daeba377e.local 43271 typ host generation 0 network-cost 999\r\na=candidate:4156233652 1 udp 1677729535 117.207.43.69 43271 typ srflx raddr 0.0.0.0 rport 0 generation 0 network-cost 999\r\na=ice-ufrag:EV3N\r\na=ice-pwd:aEfq1TO9GBLyyET0xXzZXug5\r\na=fingerprint:sha-256 82:5F:DD:D3:5A:BE:17:9F:9D:66:EC:3E:BA:FU:CU:17:20:8E:CD:13:89:E3:8B:5C:55:AE:87:A2:25:D4:19:AA\r\na=setup:actpass\r\na=mid:0\r\na=sctp-port:5000\r\na=max-message-size:262144\r\n"
        //                  });
        //                  let tojson=serde_json::to_string(&sdp_offer).unwrap();
        //                 //  print!("{:?}",tojson);
        //                 let ddata=addtoquickfetch("345".to_string(),tojson ).unwrap();
        //                 print!("{:?}",ddata);
        //                 let data=getfromquickfetch(uname).unwrap().value;
        //                 println!("{:?}",data);
        // let jdata:Value=serde_json::from_str(&data).unwrap();
        // println!("{:?}",jdata);

        // println!("{:?}",adddatatouser("vis".to_string(), "google.com".to_string()));
        // println!("{:?}",printdata());
    // }
    // else{
    //     println!("doesn't exist");
    // }
    
}
pub fn printdata()-> Result<String,()>{
    let pool=pscaleread();
    let _salt = match env::var("SALT") {
        Ok(salt) => salt,
        Err(e) => {
            eprintln!("Error reading SALT environment variable: {}", e);
            return Err(());
        }
    };

    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let results:Vec<Row> = match _conn.query(format!("SELECT * from urls")) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error querying database: {}", e);
            return Err(());
        }
    };
    let mut svec=String::new();
    for eacha in &results{

        svec.push_str(&format!("{:?}",parse_row_as_data("all".to_string(),eacha.clone())));
    }
    Ok(svec)
}
// use diesel::{Connection as dieselconn, MysqlConnection};
// fn getdbconn() -> diesel::MysqlConnection {
//     let url = env::var("DATAR").unwrap();

//     MysqlConnection::establish(&url).unwrap()
// }

#[test]
fn tryoute(){
    dotenv().ok();
    // pscaleread();
    print!("{:?}",printeuser("".to_string(), "".to_string()).unwrap());
    
}
pub fn printeuser(uid:String,_pswd:String)-> Result<Eachuser,()>{

    let salt = match env::var("SALT") {
        Ok(salt) => salt,
        Err(e) => {
            eprintln!("Error reading SALT environment variable: {}", e);
            return Err(());
        }
    };

// let mut conn = getdbconn();
//     let mut query_str = format!(
//         "SELECT * FROM urls WHERE uid = UNHEX(MD5('{}{}')) ",
//         uid,salt
//     );
// let res=diesel::sql_query(query_str)
//             // .execute(&mut conn)
//             .load::<eachuser>(&mut conn)
//             .expect("Not found");
//         // print!("{:?}",res);
//             Ok(res.get(0).unwrap().clone())

    let pool=pscaleread();


    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let qtosend=format!("SELECT * from urls WHERE uid=UNHEX(MD5('{}{}'))",uid,salt);
    println!("{}",qtosend);
    let results:Vec<Row> = match _conn.query(qtosend) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error querying database: {}", e);
            return Err(());
        }
    };
    
    Ok(parse_row_as_data(uid,match results.get(0) {
        Some(row) => row.clone(),
        None => {
            eprintln!("No results found for user");
            return Err(());
        }
    }))
    // Ok(Eachuser { id: "".to_string(), url: "".to_string() })
}

pub fn getfromquickfetch(id:String)-> Result<Eachredisentry,()>{
    let pool=pscaleread();
//SELECT value FROM urls WHERE id = 'your-uuid';

    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let results:Vec<Row> = match _conn.query(format!("SELECT value from redis WHERE id='{}'",id)) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error querying database: {}", e);
            return Err(());
        }
    };
    let first_row = match results.get(0) {
        Some(row) => row.clone(),
        None => {
            eprintln!("No results found for id: {}", id);
            return Err(());
        }
    };
    println!("{:?}",first_row);
    Ok(parse_value_from_data(first_row))
}
pub fn adddatatouser(uid:String,datatoadd:String)-> Result<String,()>{
    let pool=pscalewrite();
    let salt = match env::var("SALT") {
        Ok(salt) => salt,
        Err(e) => {
            eprintln!("Error reading SALT environment variable: {}", e);
            return Err(());
        }
    };

    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let results:Vec<Row> = match _conn.exec("UPDATE urls SET url = JSON_ARRAY_APPEND(url, '$', ?) WHERE id=UNHEX(MD5(?));",(datatoadd,format!("{}{}",uid,salt))) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            return Err(());
        }
    };
    
    Ok(format!("{:?}",results))
}
#[test]
fn testcreate(){
    dotenv().ok();
    createuser("test".to_string(),"aiven".to_string()).unwrap();
}
pub fn createuser(uid:String,_password:String)-> Result<String,()>{
    let pool=pscalewrite();
    let salt = match env::var("SALT") {
        Ok(salt) => salt,
        Err(e) => {
            eprintln!("Error reading SALT environment variable: {}", e);
            return Err(());
        }
    };

    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let results:Vec<Row> = match _conn.exec("INSERT INTO urls (uid,url) VALUES (UNHEX(MD5(?)),JSON_ARRAY());",(format!("{}{}",uid,salt),)) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            return Err(());
        }
    };
    
    Ok(format!("{:?}",results))
}
pub fn addtoquickfetch(id:String,value:String)-> Result<String,()>{
    let pool=pscalewrite();

    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let results:Vec<Row> = match _conn.exec("REPLACE INTO redis (id,value) VALUES (?,?);",(id,value)) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            return Err(());
        }
    };
    
    Ok(format!("{:?}",results))
}
pub fn checklogin(uid:String,_password:String)-> Result<String,()>{
    let pool=pscalewrite();
    let salt = match env::var("SALT") {
        Ok(salt) => salt,
        Err(e) => {
            eprintln!("Error reading SALT environment variable: {}", e);
            return Err(());
        }
    };

    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let results:Vec<Row> = match _conn.exec("SELECT * FROM urls WHERE uid = UNHEX(MD5(?));",(format!("{}{}",uid,salt),)) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            return Err(());
        }
    };
    if !results.is_empty(){
        Ok("Success".to_string())
    }
    else{
        Err(())
    }
    
}
pub fn deleteuser(uid:String,_password:String)-> Result<String,()>{
    let pool=pscalewrite();
    let salt = match env::var("SALT") {
        Ok(salt) => salt,
        Err(e) => {
            eprintln!("Error reading SALT environment variable: {}", e);
            return Err(());
        }
    };

    let mut _conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error getting database connection: {}", e);
            return Err(());
        }
    };
    let results:Vec<Row> = match _conn.exec("DELETE FROM urls WHERE uid=UNHEX(MD5(?));",(format!("{}{}",uid,salt),)) {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error executing query: {}", e);
            return Err(());
        }
    };
    
    Ok(format!("{:?}",results))
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
    let starter = pokemons.choose(&mut rand::thread_rng()).unwrap_or(&"Pikachu");
    starter.to_string()
}

#[test]
//  pub async fn main()-> Result<(), Box<dyn std::error::Error>>{
 pub fn testdata()-> Result<(), Box<dyn std::error::Error>>{

    dotenv().ok();
    println!("{:?}",printdata());
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
// #[test]
// fn datetest(){
//     let g=match NaiveDateTime::parse_from_str("2023-05-12T15:01:34+05:30","%Y-%m-%dT%H:%M:%S%z") {
//         Ok(dt) => dt,
//         Err(e) => {
//             eprintln!("Error parsing datetime: {}", e);
//             return;
//         }
//     };
                                
//     // let g1=DateTime::parse_from_str("2022-12-06T18:31:45","%Y-%m-%dT%H:%M:%S")
//     //                                 .unwrap();

//     let ndt = match NaiveDateTime::parse_from_str("2022-12-06T18:31:45Z", "%Y-%m-%dT%H:%M:%SZ") {
//         Ok(dt) => dt,
//         Err(e) => {
//             eprintln!("Error parsing datetime: {}", e);
//             return;
//         }
//     };

//                                     // .with_timezone(&FixedOffset::east_opt(5*3600+30*60).unwrap());
// }

