//We need serde for serializing data
//we need ldap3 for connecting to the UCI directory
//We need toml for easy parsing of our config file

use serde::{Serialize, Deserialize};
use ldap3::{LdapConn, Scope, SearchEntry, LdapConnSettings};
use ldap3::result::Result;
use toml;
use std::fs::{self, read};
use std::env;

// #[derive(Deserialize)]
// struct Data{
//     config: Config

// }

#[derive(Deserialize,Debug)]
struct Config{
    hostname: String,
    rdn: String,
    password: String,
    port: i32
    
}




//Create the user fields we will grab from the ldap server
struct User{
    campus_id: i32,
    email: String,
    address: String,
    org: String,
    zip: String,
    state: String,
    title: String,
    //Display Name
    name: String,
    last_name: String,
    phone: String,
    department: String,
    uci_primary_title_code: i32,
    //UciNetID string
    uid: String,
    uci_primary_cto_code: i32,
    uci_hr_status: String
}






fn main() {

    //open config file
    let config:Config = read_config("config.toml");


    //Open an LDAP connection using the config
    get_users(&config, "uciLevel3DepartmentID=IR9014")
    


}

fn read_config(file_path:&str) -> Config{

    //Error Messages
    let config_format_error:&str = 
    
    "hostname = \"ldap-auth.oit.uci.edu\"\n
    rdn = \"uid=<USER ACCOUNT>,ou=people,dc=uci,dc=edu\"\n
    password = \"<YOUR SECRET PASSWORD HERE>\"\n
    port = 636
    ";

    //Read config file
    let contents = fs::read_to_string("config.toml")
        .expect("Unable to open config.toml, check permissions and if the file exists.");

    
    //Parse toml string into our config struct
    let config: Config = toml::from_str(contents.as_str()).expect(format!("Failed to parse config.toml, check the format of the configuration.\n {}", config_format_error).as_str());

    return config;
}

fn get_users(config: &Config, filter: &str)/* -> Vec<User>*/{

    //init return variable
    let mut user_list:Vec<User>;

    let ldap_options = LdapConnSettings::new();
    ldap_options.set_starttls(true);

    //connect to LDAP
    let mut ldap = LdapConn::with_settings(ldap_options, config.hostname.as_str()).unwrap();
    //search and save result
    let rs = ldap.search(
        config.rdn.as_str(),
        Scope::Subtree,
        filter, 
        vec![
            "campus_id",
            "email",
            "address",
            "org",
            "zip",
            "state",
            "title",
            "name",
            "last_name",
            "phone",
            "department",
            "uci_primary_title_code",
            "uid",
            "uci_primary_cto_code",
            "uci_hr_status"]).unwrap();

            for entry in rs.0 {
                    let 
                    println!("Adding {:?} to the user list!\n", SearchEntry::construct(user));
                }
            
           
    


    return user_list;

}

fn write_CSV(user_list: Vec<User>, file_path:&str){

}

fn write_JSON(user_list: Vec<User>, file_path:&str){

}