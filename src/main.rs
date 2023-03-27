//We need serde for serializing data
//we need ldap3 for connecting to the UCI directory
//We need toml for easy parsing of our config file

use serde::{Serialize, Deserialize};
use ldap3::{LdapConn, Scope, SearchEntry};
use ldap3::result::Result;
use toml;
use std::fs;
use std::env;

#[derive(Deserialize)]
struct Data{
    config: Config

}

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

    //Read config file
    let contents = fs::read_to_string("config.toml")
        .expect("Should have been able to read the file");

    println!("{}", contents);

    let config: Config = toml::from_str(contents.as_str());

    println!("{:?}", config);

}
