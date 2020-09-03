use std::io;
use rand::Rng;

fn opt_menu()-> i32{
    println!("Seja bem-vindo ao Pedra,papel e tesoura \nSelecione uma opção digitando o número equivalente:\n[1] – Jogar com a outro Jogador\n[2] – Jogar com a CPU\n[0] – Sair do programa\nDigite o valor da opção selecionada: ");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let choice:i32 = guess
                    .trim()
                    .parse()
                    .expect("Opção incorreta");
    return choice   
        
}
fn opt_jogador(){
    println!("Jogador 1, Selecione a sua altenativa \nSelecione uma opção digitando o número equivalente:\n[2] – Tesoura\n[1] – Papel\n[0] – Pedra\nDigite o valor da opção selecionada: ");

    println!("Please input your guess.");

    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    let choicex:i32 = x
                    .trim()
                    .parse()
                    .expect("Opção incorreta");

    println!("Jogador 1, Selecione a sua altenativa \nSelecione uma opção digitando o número equivalente:\n[2] – Tesoura\n[1] – Papel\n[0] – Pedra\nDigite o valor da opção selecionada: ");

    println!("Please input your guess.");

    let mut y = String::new();

    io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");
    let choicey:i32 = y
                    .trim()
                    .parse()
                    .expect("Opção incorreta");   
    
        if choicex==1 && choicey==2 {
            println!("Jogador 2 venceu");
        } else if choicex==1 && choicey==0 {
            println!("jogador 1 venceu");
        } else if choicex==2 && choicey==0 {
            println!("jogador 2 venceu");
        } else if choicex==2 && choicey==1 {
            println!("jogador 1 venceu");
        } else if choicex==choicey {
            println!("empate");
        }else if choicex==0 && choicey==2 {
            println!("jogador 1 venceu");
        } else if choicex==0 && choicey==1 {
            println!("jogador 2 venceu");
        } else if choicex == choicey{
            println!("Empate")
        }
        
}

fn opt_computador(){
    println!("Jogador 1, Selecione a sua altenativa \nSelecione uma opção digitando o número equivalente:\n[2] – Tesoura\n[1] – Papel\n[0] – Pedra\nDigite o valor da opção selecionada: ");

    println!("Please input your guess.");

    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    let choicex:i32 = x
                    .trim()
                    .parse()
                    .expect("Opção incorreta");

    let mut rng = rand::thread_rng();

    let choicey: i32 = rng.gen_range(-1, 3);           
                        
                        
   
                
        if choicex==1 && choicey==2 {
            println!("computador venceu");
        } else if choicex==1 && choicey==0 {
            println!("jogador 1 venceu");
        } else if choicex==2 && choicey==0 {
            println!("computador venceu");
        } else if choicex==2 && choicey==1 {
            println!("jogador 1 venceu");
        } else if choicex==choicey {
            println!("empate");
        }else if choicex==0 && choicey==2 {
            println!("jogador 1 venceu");
        } else if choicex==0 && choicey==1 {
            println!("computador venceu");
        } else if choicex == choicey{
            println!("Empate")
        }
}

fn main(){
    print!("Bem vindo ao PPT");
    let choice :i32=opt_menu();
    
    match choice {
        1=> opt_jogador(),
        2=> opt_computador(),
        3=> print!("opção 03"),
        _=> print!("resto"),
    };
    
}
