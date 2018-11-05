mod nmap_manager;
mod macvendor_manager;
mod database_manager;

fn main() {
    //let host_list_ip_mac = nmap_manager::execute_nmap( "192.168.1.1".to_string() );
    //let host_list_ip_mac_manofacturer = macvendor_manager::get_vendors( host_list_ip_mac );
    //println!("{:?}",host_list_ip_mac_manofacturer)

    let db_params: database_manager::Connection_params = database_manager::Connection_params{
        user: "postgres".to_string(),
        password: "postgres".to_string(),
        db_name: "rust_test".to_string(),
        host: "localhost".to_string(),
        port: 5432
    };
    
    let conn = database_manager::get_connection( &db_params );

    database_manager::write_database( &conn );
    database_manager::record_macvendors_log( &conn );
}
