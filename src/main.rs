extern crate chrono;

use chrono::NaiveDateTime;
use chrono::Duration;
use chrono::prelude::*;

mod nmap_manager;
mod macvendor_manager;
mod lib;


fn main() {
    
    let db_params: lib::ConnectionParams = lib::ConnectionParams{
        user: "postgres".to_string(),
        password: "postgres".to_string(),
        db_name: "rust_test".to_string(),
        host: "localhost".to_string(),
        port: 5432
    };
    
    let conn = lib::get_connection( &db_params );
    lib::write_database( &conn );
    let host_list_ip_mac = nmap_manager::execute_nmap( "192.168.1.1".to_string() );

    let current_datetime:NaiveDateTime = Utc::now().naive_utc();
    let last_db_update_datetime:NaiveDateTime = lib::get_last_macvendors_logs_record( &conn );

    if  (current_datetime - last_db_update_datetime) > Duration::days(1) {
        macvendor_manager::update_vendros_db();
        lib::record_macvendors_log( &conn );
    }

    let host_list_ip_mac_manofacturer = macvendor_manager::get_vendors( host_list_ip_mac );

    println!( "{:#?}",host_list_ip_mac_manofacturer );
}
