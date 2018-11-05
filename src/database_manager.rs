extern crate postgres;

use self::postgres::{Connection, TlsMode};

struct macvendors_logs {
    id: i32,
    datetime: String
}

struct nmap_logs {
    id: i32,
    ip: String,
    mac: String,
    vendor: String,
    datetime: String
}

struct config {
    id: i32,
    alias: String,
    value: String,
    alia: String,
    datetime: String
}

pub fn write_database( user:String, password:String, database_name:String, host:String, port:i32 ){

    let uri_conection = "postgres://".to_string()
                        +  &user + &":" + &password 
                        + &"@" + &host + &":" + &port.to_string() 
                        + &"/" + &database_name;
    let conn = Connection::connect( uri_conection, TlsMode::None).unwrap();

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