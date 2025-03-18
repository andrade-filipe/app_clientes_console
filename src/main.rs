mod structs;
mod display;

use structs::cliente::Cliente;
use display::menu as menu;

fn main() {
    let mut clientes:Vec<Cliente> = Vec::new();
    menu::menu(&mut clientes);
}
