extern crate postgres;

use self::postgres::{Connection, TlsMode};

pub struct Connection_params {
    pub user: String,
    pub password: String, 
    pub db_name: String,
    pub host: String,
    pub port: i32
}

pub struct Macvendors_logs {
    pub id: i32,
    pub datetime: String
}

pub struct Nmap_logs {
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

pub fn get_connection( params:&Connection_params )->Connection{
    let uri_conection = "postgres://".to_string()
                        +  &params.user + &":" + &params.password 
                        + &"@" + &params.host + &":" + &params.port.to_string() 
                        + &"/" + &params.db_name;
    Connection::connect( uri_conection, TlsMode::None).unwrap()
}

pub fn write_database( conn:&Connection ){

    conn.execute(
            "CREATE TABLE macvendors_logs (
                id          SERIAL PRIMARY KEY,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        );
    
    conn.execute(
            "CREATE TABLE nmap_logs (
                id          SERIAL PRIMARY KEY,
                ip          VARCHAR NULL,
                mac         VARCHAR NULL,
                vendor      VARCHAR NULL,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        );
    
    conn.execute(
            "CREATE TABLE config (
                id          SERIAL PRIMARY KEY,
                alias       VARCHAR NULL,
                value       VARCHAR NULL,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        );
    
    conn.execute(
            "CREATE TABLE known_macs (
                id          SERIAL PRIMARY KEY,
                mac         VARCHAR NULL,
                alias       VARCHAR NULL,
                extra       VARCHAR NULL,
                datetime    timestamp DEFAULT current_timestamp
            )", 
            &[]
        );

}

pub fn record_macvendors_log( conn:&Connection ){
    conn.execute("INSERT INTO macvendors_logs (datetime) VALUES (NOW())", &[]);
}