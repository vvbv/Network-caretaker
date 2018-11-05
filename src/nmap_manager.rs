use std::process::Command;

pub fn execute_nmap( default_route:String )->Vec<(String, String)>{

    let mut to_return = Vec::new();

    let output = Command::new( "nmap" )
                    .arg( "-sP" )
                    .arg( "-n" )
                    .arg( default_route + "/24" )
                    .output()
                    .expect( "failed to execute process" );

    let output_string = String::from_utf8_lossy( &output.stdout );
    let lines: Vec< &str > = output_string.split( "\n" ).collect();

    for _i in 0..lines.len(){
        if lines[ _i ].to_string().find( "Nmap scan report for" ) == Some( 0 ) {

            let ip_line: Vec<&str> = lines[_i].split(" ").collect();
            let ip: String = ip_line[ 4 ].to_string();
            let mut mac: String = "".to_string();
            
            if lines[ _i + 2 ].to_string().find( "MAC" ) == Some( 0 ){
                let mac_line: Vec<&str> = lines[ _i + 2 ].split( " " ).collect();
                mac = mac_line[ 2 ].to_string();
            }
            let pair = ( ip, mac );
            to_return.push( pair );
        }
    }
    to_return
}