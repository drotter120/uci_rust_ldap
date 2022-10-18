
//Create the user fields we will grab from the ldap server
struct user{
    campusid: i32,
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
    println!("Hello, world!");
}
