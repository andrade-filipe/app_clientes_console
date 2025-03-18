use crate::{display::{client_service::{alterar_cliente, excluir_cliente, incluir_cliente, listar_clientes}, utils::ler_dados_int}, structs::cliente::Cliente};

use super::utils::limpar_tela;

pub fn menu(clientes: &mut Vec<Cliente>){
    loop {
        limpar_tela();
        println!("1 - Cadastrar cliente");
        println!("2 - Alterar cliente");
        println!("3 - Excluir cliente");
        println!("4 - Listar clientes");
        println!("0 - Sair");

        let opcao = ler_dados_int();

        match opcao {
            1 => incluir_cliente(clientes),
            2 => alterar_cliente(clientes),
            3 => excluir_cliente(clientes),
            4 => listar_clientes(clientes),
            0 => break,
            _ => continue,
        }
    }
}