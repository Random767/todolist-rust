use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::process::exit;

fn main() {
    println!("Iniciando Gerenciador de tarefas");
    println!("Criado em Rust por GRandom :D");
    start();
}

fn start() {
    // Função responsável pela configuração e 
    // configuração do código
    let mut tasks: Vec<String> = Vec::new();
    let _ = handle_user_inputs(&mut tasks);
}

fn handle_user_inputs(tasks: &mut Vec<String>) -> Result<()>{
    let mut rl = DefaultEditor::new()?;
    // Um "?" no final da linha significa que
    // o comando pode retornar um erro
    
    #[cfg(feature = "with-file-history")]
    // Essa linha de cima basicamente diz ao  
    // compilador que a flag with-file-history 
    // precisa estar ativa

    if rl.load_history("command_history.txt").is_err() {
        println!("Histórico de comandos não encontrado");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.len() == 0 {
                    println!("");
                    //return;
                } else {
                    command_handler(tasks, line.as_str());
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CRTL-D");
                break
            },
            Err(err) => {
                print!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("command_history.txt");
    Ok(())
}

fn command_handler(tasks: &mut Vec<String>, pure_input: &str) {
    let mut args_splited: Vec<String> = Vec::new();
    for word in pure_input.split_whitespace() {
        args_splited.push(word.to_string());
    }

    let command = args_splited.remove(0);
    println!("Comando recebido: {}", command);

    let argument_treated = args_splited.join(" ");

    if command == "add" {
        if args_splited.len() == 0 {
            return println!("O nome da tarefa não pode ser vazio");
        }
        let index = find_index(tasks, argument_treated.clone());
        if index != -1 {
            println!("Essa tarefa já existe");
        }
        create_task(tasks, argument_treated);
        println!("Tarefa adicionada");
    } else if command == "list" {
        let mut i = 0;
        while i < tasks.len() {
            println!("{}. {}", i, tasks[i]);
            i += 1;
        }
    } else if command == "exit" {
        println!("Saindo...");
        exit(0);
    } else {
        println!("[Erro] Comando não encontrado");
    }
}

fn create_task(tasks: &mut Vec<String>, name: String) {
    tasks.push(name);
}

fn find_index(tasks: &mut Vec<String>, to_find: String) -> i32 {
    // Criar uma função que procure 
    // pelo nome da tarefa no vetor 
    // e retorne a index do valor

    println!("To find: {}", to_find);
    if tasks.len() == 0 {
        return -1;
    }
    let index = tasks.iter().position(|r| r.to_string() == to_find).unwrap();
    println!("Index encontrada: {}", index);
    return index as i32;
}

// TO-DO: Criar um script para separar 
// o comando do usuário para o argumento

// TO-DO: Criar os comandos :D
// lembre-se de estudar vetores
// ou estudar melhor para ver se 
// armazenar lista de afazeres 
// em vetores é uma boa opção
//
//Nota 04/03/2024: Consertar erros envolvendo 
//o vetor "tasks" na função find_index
//
