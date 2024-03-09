use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::process::exit;

fn main() {
    println!("Iniciando Gerenciador de tarefas");
    println!("Criado em Rust por GRandom :D");
    start();
}

#[derive(Default)]
struct Task {
    name: String,
    is_marked: bool,
}

fn start() {
    let mut tasks: Vec<Task> = Vec::new();
    let _ = handle_user_inputs(&mut tasks);
}

fn handle_user_inputs(tasks: &mut Vec<Task>) -> Result<()>{
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
                if line.len() != 0 {
                    rl.add_history_entry(line.as_str());
                    let mut args_splited: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
                    let command = args_splited.remove(0).to_string();
                    let argument_treated = args_splited.join(" ");
                    command_handler(tasks, &command, &argument_treated);
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

fn command_handler(tasks: &mut Vec<Task>, command: &str, args: &str) {
    match command {
        "add" => {
            if args.is_empty() {
                return println!("O nome da tarefa não pode ser vazio");
            }
            let index = find_index(tasks, args.clone());
            if index != -1 {
                println!("Essa tarefa já existe");
            } else {
                create_task(tasks, args);
                println!("Tarefa adicionada");
            }
        }
        "list" => {
            let mut i = 0;
            while i < tasks.len() {
                println!("{}. {}", i, tasks[i].name);
                i += 1;
            }
        }
        "remove" => {
            if args.is_empty() {
                return println!("O nome da tarefa a ser removida não pode ser vazio");
            }
            let index = find_index(tasks, args.try_into().unwrap());
            if index == -1 {
                println!("Tarefa não encontrada");
            } else {
                let task_name = tasks.remove(index.try_into().unwrap()).name;
                println!("Tarefa '{}' foi removida", task_name);
            }
        }
        "exit" => {
            println!("Saindo...");
            exit(0);
        }
        _ => {
            println!("[Erro] Comando não encontrado");
        }
    }
}

fn create_task(tasks: &mut Vec<Task>, name: &str) {
    tasks.push(Task {
        name: name.to_string(),
        is_marked: false,
    });
}

fn find_index(tasks: &mut Vec<Task>, to_find: &str) -> i32 {
    if tasks.len() == 0 {
        return -1;
    }

    let index = match tasks.iter().position(|r| r.name.to_string() == to_find) {
        Some(index) => index,
        None => {
            return - 1
        }
    };
    return index as i32;
}

