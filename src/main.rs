mod nmap_manager;
mod macvendor_manager;
mod database_manager;

fn main() {
    //let host_list_ip_mac = nmap_manager::execute_nmap( "192.168.1.1".to_string() );
    //let host_list_ip_mac_manofacturer = macvendor_manager::get_vendors( host_list_ip_mac );
    //println!("{:?}",host_list_ip_mac_manofacturer)
    database_manager::write_database( 
        "postgres".to_string(), 
        "postgres".to_string(), 
        "rust_test".to_string(),
        "localhost".to_string(),
        5432 );
}
