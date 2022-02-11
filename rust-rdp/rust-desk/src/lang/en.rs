lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Your desktop can be accessed with this ID and password."),
        ("connecting_status", "Connecting to the RustDesk network..."),
        ("not_ready_status", "Not ready. Please check your connection"),
        ("id_change_tip", "Only a-z, A-Z, 0-9 and _ (underscore) characters allowed. The first letter must be a-z, A-Z. Length between 6 and 16."),
        ("install_tip", "Due to UAC, RustDesk can not work properly as the remote side in some cases. To avoid UAC, please click the button below to install RustDesk to the system."),
        ("config_acc", "In order to control your Desktop remotely, you need to grant RustDesk \"Accessibility\" permissions."),
        ("config_screen", "In order to access your Desktop remotely, you need to grant RustDesk \"Screen Recording\" permissions."),
        ("agreement_tip", "By starting the installation, you accept the license agreement."),
        ("not_close_tcp_tip", "Don't close this window while you are using the tunnel"),
        ("setup_server_tip", "For faster connection, please set up your own server"),
        ("Auto Login", "Auto Login (Only valid if you set \"Lock after session end\")"),
        ("whitelist_tip", "Only whitelisted IP can access me"),
        ("whitelist_sep", "Seperated by comma, semicolon, spaces or new line"),
        ("Wrong credentials", "Wrong username or password"),
        ("invalid_http", "must start with http:// or https://"),
        ("install_daemon_tip", "For starting on boot, you need to install system service."),
    ].iter().cloned().collect();
}
