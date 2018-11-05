extern crate postgres;

use self::postgres::{Connection, TlsMode};
use std::num::ParseIntError;

pub struct ConnectionParams {
    pub user: String,
    pub password: String, 
    pub db_name: String,
    pub host: String,
    pub port: i32
}

pub struct MacvendorsLog {
    pub id: i32,
    pub datetime: String
}

pub struct NmapLog {
    pub id: i32,
    pub ip: String,
    pub mac: String,
    pub vendor: String,
    pub datetime: String
}

pub struct Config {
    pub id: i32,
    pub alias: String,
    pub value: String,
    pub alia: String,
    pub datetime: String
}

pub fn get_connection( params:&ConnectionParams )->Connection{
    let uri_conection = "postgres://".to_string()
                        +  &params.user + &":" + &params.password 
                        + &"@" + &params.host + &":" + &params.port.to_string() 
                        + &"/" + &params.db_name;
    Connection::connect( uri_conection, TlsMode::None).unwrap()
}

pub fn write_database( conn:&Connection ) {

    conn.execute(
            "CREATE TABLE macvendors_logs (
                id          SERIAL PRIMARY KEY,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        ).err();
    
    conn.execute(
            "CREATE TABLE nmap_logs (
                id          SERIAL PRIMARY KEY,
                ip          VARCHAR NULL,
                mac         VARCHAR NULL,
                vendor      VARCHAR NULL,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        ).err();
    
    conn.execute(
            "CREATE TABLE config (
                id          SERIAL PRIMARY KEY,
                alias       VARCHAR NULL,
                value       VARCHAR NULL,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        ).err();
    
    conn.execute(
            "CREATE TABLE known_macs (
                id          SERIAL PRIMARY KEY,
                mac         VARCHAR NULL,
                alias       VARCHAR NULL,
                extra       VARCHAR NULL,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        ).err();
}

pub fn record_macvendors_log( conn:&Connection ){
    conn.execute("INSERT INTO macvendors_logs (datetime) VALUES (NOW())", &[]).err();
}

pub fn record_nmap_log( log:NmapLog, conn:&Connection ){
    conn.execute("INSERT INTO nmap_logs (ip, mac, vendor) VALUES ($1, $2, $3)", &[ &log.ip, &log.mac, &log.vendor ]).err();
}

pub fn get_last_macvendors_logs_record( conn:&Connection )->chrono::NaiveDateTime{
    let stmt = conn.prepare("SELECT id, extract('epoch' from datetime)::bigint AS datetime FROM macvendors_logs ORDER BY datetime DESC LIMIT 1").unwrap();
    let mut last_update_datetime:chrono::NaiveDateTime =  chrono::NaiveDateTime::from_timestamp(0, 0);
    for row in &stmt.query(&[]).unwrap() {
        last_update_datetime = chrono::NaiveDateTime::from_timestamp(row.get(1), 0);
    }
    last_update_datetime
}