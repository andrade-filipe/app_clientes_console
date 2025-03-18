use crate::{display::utils::esperar, structs::cliente::Cliente};

use super::utils::{ler_dados, ler_dados_int};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>){
    let mut cliente: Cliente = Cliente::default();

    cliente.id = clientes.len() + 1;

    digitar_dados_cliente(&mut cliente);

    clientes.push(cliente);
    println!("Sucesso!");
    esperar(1);
}

pub fn alterar_cliente(clientes: &mut Vec<Cliente>){
    if clientes_esta_vazio(clientes) {
        return;
    }

    println!("Id do Cliente: ");
    let id = ler_dados_int();
    
    let old_cliente = clientes.iter().enumerate().find(| (_, cliente) | cliente.id == id);

    if let Some((index, cliente)) = old_cliente {
        mostrar_cliente(cliente);
        println!("{}", "=".to_string().repeat(30));
        digitar_dados_cliente(&mut clientes[index]);
    }

    println!("Cliente Alterado");
    esperar(1);
}

pub fn excluir_cliente(clientes: &mut Vec<Cliente>){
    if clientes_esta_vazio(clientes) {
        return;
    }

    println!("Id do Cliente: ");
    let id = ler_dados_int();
    
    let old_cliente = clientes.iter().enumerate().find(| (_, cliente) | cliente.id == id);

    if let Some((index, cliente)) = old_cliente {
        mostrar_cliente(cliente);
        clientes.remove(index);
    }

    println!("Cliente Deletado");
    esperar(1);
}

pub fn listar_clientes(clientes: &mut Vec<Cliente>){
    if clientes_esta_vazio(clientes) {
        return;
    }

    for cliente in clientes{
        println!("{}", "=".to_string().repeat(30));
        mostrar_cliente(cliente);
    }

    esperar(3);
}

fn digitar_dados_cliente(cliente: &mut Cliente){
    println!("Nome:");
    cliente.nome = ler_dados();
    println!("CPF:");
    cliente.cpf = ler_dados();
    println!("Endereço:");
    cliente.endereco = ler_dados();
}

fn mostrar_cliente(cliente: &Cliente){
    println!("ID: {}", cliente.id);
    println!("NOME: {}", cliente.nome);
    println!("CPF: {}", cliente.cpf);
    println!("ENDERECO: {}", cliente.endereco);
}

fn clientes_esta_vazio(clientes: &Vec<Cliente>) -> bool{
    if clientes.len() == 0 {
        println!("Lista Vazia");
        esperar(1);
        return true;
    }
    return false;
}