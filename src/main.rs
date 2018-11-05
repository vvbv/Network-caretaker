mod nmap_manager;
mod macvendor_manager;

fn main() {
    let host_list_ip_mac = nmap_manager::execute_nmap( "192.168.1.1".to_string() );
    let host_list_ip_mac_manofacturer = macvendor_manager::get_vendors( host_list_ip_mac );
    println!("{:?}",host_list_ip_mac_manofacturer)
}
