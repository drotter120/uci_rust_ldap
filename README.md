# uci_rust_ldap
A simple tool I made for pulling data from UCI's ldap servers and outputting it in CSV or JSON format.

### Create a config.toml file with the following information
- hostname = "ldap-auth.oit.uci.edu"
- rdn = "uid=UCI_LDAP_USER,ou=people,dc=uci,dc=edu"
- password = "<YOUR_UCI_LDAP_USER_PASSWORD"
- port = 636



