use std::{thread::sleep, time::Duration};

pub fn limpar_tela(){
    clearscreen::clear().expect("falha ao limpar")
}

pub fn ler_dados() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Erro ao ler");

    buf.trim().to_string()
}

pub fn ler_dados_int() -> usize{
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Erro ao ler");

    buf.trim().parse().expect("Erro ao Converter para inteiro")
}

pub fn esperar(secs: u64){
    sleep(Duration::from_secs(secs));
}