use std::io;

fn main() {
    choose();
}

fn choose() {

    println!("\nEscolha uma opção de conversão: \n 1. Binario \n 2. Hexadecimal");
    let mut choose = String::new();

    io::stdin().read_line(&mut choose)
        .expect("Failed to read line");

    let choose: u32 = choose.trim()
    .parse()
    .expect("need to be a number");

    match choose {
        1 => binarie(),
        2 => hex(),
        _ => println!("Choose a valid option"),
        
    }
}

fn binarie(){
    loop{
        println!("\nQual numero você quer converter para binario?\n Digite um número: ");
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Falha ao ler número");

        let number: u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("O numero {number} em binario é: {:b}", number);
    }
}

fn hex(){
    loop {
        println!("\nQual numero você quer converter para hexadecimal?\nDigite um número: ");
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Falha ao ler número");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("O numero {} em hexadecimal é: {:X}", number, number);
    }
}