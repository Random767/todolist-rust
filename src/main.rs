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

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.len() != 0 {
                    let _ = rl.add_history_entry(line.as_str());
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
    Ok(())
}

fn command_handler(tasks: &mut Vec<Task>, command: &str, args: &str) {
    match command {
        "add" => handle_add_task(tasks, args),
        "list" => handle_list_task(&tasks),
        "remove" => handle_remove_task(tasks, args),
        "mark" => handle_mark_task(tasks, args),
        "unmark" => handle_unmark_task(tasks, args),
        "exit" => handle_exit_task(),
        _ => {
            println!("[Erro] Comando não encontrado");
        }
    }
}

fn handle_add_task(tasks: &mut Vec<Task>, args: &str) {
    if args.is_empty() {
        return println!("O nome da tarefa não pode ser vazio");
    }

    let index = find_index(tasks, args);
    match index {
        Some(_) => println!("Essa tarefa já existe"),
        None => {
            create_task(tasks, args);
            println!("Tarefa adicionada");
        },
    }
}

fn handle_list_task(tasks: &Vec<Task>) {
    let mut i = 0;
    while i < tasks.len() {
        if tasks[i].is_marked {
            println!("{}. \x1b[9m{}\x1b[0m", i+1, tasks[i].name);
        } else {
            println!("{}. {}", i+1, tasks[i].name);
        }
        i += 1;
    }
}

fn handle_remove_task(tasks: &mut Vec<Task>, args: &str) {
    if args.is_empty() {
        return println!("O nome da tarefa a ser removida não pode ser vazio");
    }
    let index = find_index(tasks, args.try_into().unwrap());
    match index {
        Some(i) => {
            let task_name = tasks.remove(i).name;
            println!("Tarefa '{}' foi removida", task_name);

        },
        None => println!("Tarefa não encontrada"),
    }
}

fn handle_mark_task(tasks: &mut Vec<Task>, args: &str) {
    if args.is_empty() {
        return println!("Não posso marcar o nada");
    }
    let index = find_index(tasks, args.try_into().unwrap());
    match index {
        Some(i) => {
            tasks[i].is_marked = true;
            println!("Tarefa marcada com sucesso");
        }
        None => {
            println!("Tarefa não encontrada");
        }
    }
}
fn handle_unmark_task(tasks: &mut Vec<Task>, args: &str) {
    if args.is_empty() {
        return println!("Não posso desmarcar o nada");
    }
    let index = find_index(tasks, args.try_into().unwrap());
    match index {
        Some(i) => {
            tasks[i].is_marked = false;
            println!("Tarefa desmarcada com sucesso");
        }
        None => {
            println!("Tarefa não encontrada");
        }
    }
}

fn handle_exit_task() {
    println!("Saindo...");
    exit(0);
}

fn create_task(tasks: &mut Vec<Task>, name: &str) {
    tasks.push(Task {
        name: name.to_string(),
        is_marked: false,
    });
}

fn find_index(tasks: &Vec<Task>, to_find: &str) -> Option<usize> {
    tasks.iter().position(|r| r.name.to_string() == to_find)
}

