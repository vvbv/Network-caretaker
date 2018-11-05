use std::process::Command;
use lib;

pub fn update_vendros_db()->std::process::ExitStatus{
    let output = Command::new( "macvendor" )
                    .arg( "-no--update" )
                    .arg( "0" )
                    .output()
                    .expect( "failed to execute process" );
    output.status
}

pub fn get_vendors( hosts:Vec<(String, String)> ) -> Vec<(String, String, String)>{
    let mut to_return:Vec<(String, String, String)> = vec![];
    let mut first:bool = true;
    for _host in hosts.iter(){
        let (ip, mac) = _host;
        if mac != "" {
            let output = Command::new( "macvendor" )
                            .arg( "--no-update" )
                            .arg( mac )
                            .output()
                            .expect( "failed to execute process" );
            let string_out = String::from_utf8_lossy( &output.stdout );
            let string_tok:Vec<&str> = string_out.split(",").collect();
            let manofacturer = string_tok[1].trim();
            let tuple = (ip.to_string(), mac.to_string(), manofacturer.to_string());
            to_return.push(tuple);
        }else{
            to_return.push((ip.to_string(), mac.to_string(), "".to_string()));
        }
    }
    to_return
}