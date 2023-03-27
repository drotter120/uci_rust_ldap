//We need serde for serializing data
//we need ldap3 for connecting to the UCI directory
//We need toml for easy parsing of our config file

use serde::{Serialize, Deserialize};
use ldap3::{LdapConn, Scope, SearchEntry};
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
    let config:Config = readConfig("config.toml");


    //Open an LDAP connection using the config
    
    


}

fn readConfig(file_path:&str) -> Config{

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

fn getUsers(config: &Config, filter: &str) -> Vec<User>{

    //connect to LDAP
    let mut ldap = LdapConn::new(config.hostname.as_str())?;
    //search and save result
    let (rs, _res) = ldap.search(
        config.rds.as_str(), scope::Subtree, filter, ALL_ATTRIBUTES)

    

    let user_list:Vec<User>;

    return user_list;

}

fn writeCSV(user_list: Vec<User>, file_path:&str){

}

fn writeJSON(user_list: Vec<User>, file_path:&str){

}