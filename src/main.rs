use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();
        println!(
            "Escolha:\n 1. Adicionar tarefa\n 2. Atualizar tarefa \n 3. Listar tasks\n 4. Remover task\n 5. Sair");
        io::stdin().read_line(&mut input).unwrap();

        let num: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Digite um numero valido");
                continue;
            }
        };
        match num {
            1 => adicionar_task(&mut tasks),
            2 => atualizar_task(&mut tasks),
            3 => listar_task(&tasks),
            4 => remover_task(&mut tasks),
            5 => {
                println!("Saindo");
                break;
            }
            _ => println!("Invalido"),
        }
    }
}
fn adicionar_task(tasks: &mut Vec<String>) {
    let mut task: String = String::new();
    println!("Digite a tarefa: ");

    io::stdin().read_line(&mut task).unwrap();

    tasks.push(task.trim().to_string());
    println!("Task adicionada");
}

fn atualizar_task(tasks: &mut [String]) {
    if tasks.is_empty() {
        println!("Nao ha tasks para atualizar");
        return;
    }
    println!("Tasks atuais: ");
    listar_task(tasks);

    println!("Qual voce quer atualizar? ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let escolha: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Digite um numero valido");
            return;
        }
    };
    if escolha == 0 {
        println!("Numero invalido, (comeca em 1).");
        return;
    }

    let idx = escolha - 1;

    let mut novo = String::new();
    io::stdin().read_line(&mut novo).unwrap();
    let novo = novo.trim().to_string();

    match tasks.get_mut(idx) {
        Some(task) => {
            *task = novo;
            println!("Task atualizada");
        }
        None => println!("Indice invalida"),
    }
}

fn listar_task(tasks: &[String]) {
    for (i, task) in tasks.iter().enumerate() {
        println!("{} - {}", i + 1, task);
    }
}

fn remover_task(tasks: &mut Vec<String>) {
    if tasks.is_empty() {
        println!("Nao ha tasks para remover");
        return;
    }
    println!("Tasks atuais: ");
    listar_task(tasks);

    println!("Qual voce quer remover?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let escolha: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Digite um numero valido");
            return;
        }
    };
    if escolha == 0 {
        println!("Numero invalido, (comeca em 1).");
        return;
    }

    let idx = escolha - 1;

    if idx >= tasks.len() {
        println!("Indice invalido!");
        return;
    }

    tasks.remove(idx);
    println!("Task removida")
}
