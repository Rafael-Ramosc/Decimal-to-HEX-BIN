use std::io;

fn main() {
    loop {
        match choose() {
            Ok(_) => {},
            Err(e) => println!("Erro: {}", e),
        }
    }
}

fn choose() -> Result<(), &'static str> {
    println!("\nEscolha uma opção de conversão: \n 1. Binário \n 2. Hexadecimal \n 3. Sair");
    let choose = read_input()?.trim().parse::<u32>();

    match choose {
        Ok(1) => binarie()?,
        Ok(2) => hex()?,
        Ok(3) => std::process::exit(0),
        _ => println!("Escolha uma opção válida."),
    }
    Ok(())
}

fn binarie() -> Result<(), &'static str> {
    let number = read_number("binário")?;
    println!("O número {number} em binário é: {:b}", number);
    Ok(())
}

fn hex() -> Result<(), &'static str> {
    let number = read_number("hexadecimal")?;
    println!("O número {number} em hexadecimal é: {:X}", number);
    Ok(())
}

fn read_input() -> Result<String, &'static str> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Err("Falha ao ler entrada");
    }
    Ok(input)
}

fn read_number(conversion: &str) -> Result<u32, &'static str> {
    loop {
        println!("\nQual número você quer converter para {conversion}?\nDigite um número ou 'voltar' para retornar ao menu principal:");
        let input = read_input()?;
        if input.trim().eq_ignore_ascii_case("voltar") {
            break Ok(0);
        }
        match input.trim().parse::<u32>() {
            Ok(num) => break Ok(num),
            Err(_) => println!("Por favor, digite um número válido."),
        }
    }
}
