use std::fs::OpenOptions;
use std::io;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};
fn main() {
    let mut method = String::new();
    println!("Enter action you want to perform");
    io::stdin()
        .read_line(&mut method)
        .expect("Didnt receive input");
    // the &[..] syntax to create a string slice from the method
    // variable. This syntax is equivalent to method.as_str(),
    // but it's more concise. Then, we're using the match expression
    // with a reference to the string slice, which allows us to match against string literals.
    match &method.trim()[..] {
        "add" => println!("{}", add_task()),
        "remove" => println!("{}", remove_task()),
        "complete" => println!("{}",complete_task()),
        "help" => println!("{}",show_help()),
        "summary" => println!("{}",show_summary()),
        _ => println!("Invalid input type help for more info"),
    }
}

fn add_task() -> String {
    let mut index: u8 = 1;
    let task_contents = read_task();
    if task_contents.len() > 0 {
        let task = task_contents[task_contents.len() - 1].clone();
        if let Some(first_char) = task.chars().next() {
            index = first_char as u8 - b'0';

            println!("Enter your task");
            let mut task = String::new();
            io::stdin()
                .read_line(&mut task)
                .expect("Didnt receive input");
            println!("Assign a priority between 1 to 10");
            let mut priority = String::new();
            io::stdin()
                .read_line(&mut priority)
                .expect("Didnt receive input");
            let priority = priority
                .trim()
                .parse::<u8>()
                .expect("Failed to parse priority");

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("tasks.txt")
                .unwrap();
            file.write_all(format!("\n{}. {} {}", index + 1, task.trim(), priority).as_bytes())
                .unwrap();
            String::from("Task added succesfully")
        } else {
            String::from("Error: Task not found")
        }
    }else{
        println!("Enter your task");
            let mut task = String::new();
            io::stdin()
                .read_line(&mut task)
                .expect("Didnt receive input");
            println!("Assign a priority between 1 to 10");
            let mut priority = String::new();
            io::stdin()
                .read_line(&mut priority)
                .expect("Didnt receive input");
            let priority = priority
                .trim()
                .parse::<u8>()
                .expect("Failed to parse priority");

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("tasks.txt")
                .unwrap();
            file.write_all(format!("\n{}. {} {}", index, task.trim(), priority).as_bytes())
                .unwrap();
            String::from("Task added succesfully")
    }
}

fn remove_task() -> String {
    let task_contents = read_task();
    for task in &task_contents{
        println!("{:?}",task)
    }
    let mut index = String::new();
    println!("Enter task number to be removed");
    io::stdin()
        .read_line(&mut index)
        .expect("Didnt receive input");
    let delete_index = index.trim().parse::<usize>().unwrap()-1;
    let modified_elements = &task_contents[delete_index + 1..];
    let mut new_elements = Vec::new();
    for s in modified_elements {
        let mut iter = s.splitn(2, '.');
        let num_str = iter.next().unwrap();
        let rest = iter.next().unwrap();
        let new_num = num_str.parse::<u8>().unwrap() - 1;
        let new_s = format!("{}{}{}", new_num, ".", rest);
        new_elements.push(new_s);
    }
    let mut task_contents_copy = task_contents.clone();
    println!("here before{:?}", task_contents_copy);
    task_contents_copy.splice(delete_index..delete_index + new_elements.len()+1, new_elements);
    println!("here after{:?}", task_contents_copy);
    write_removed_task(&task_contents_copy).unwrap();
    String::from("Task deleted successfully")
}

fn read_task() -> Vec<String> {
    let file = File::open("tasks.txt").unwrap();
    let reader = BufReader::new(file);

    let mut tasks = Vec::new();
    for line in reader.lines() {
        tasks.push(line.unwrap());
    }
    tasks
}

fn read_completed_task() -> Vec<String> {
    let file = File::open("completed.txt").unwrap();
    let reader = BufReader::new(file);

    let mut tasks = Vec::new();
    for line in reader.lines() {
        tasks.push(line.unwrap());
    }
    tasks
}

fn complete_task() -> String {
    let task_contents = read_task();
    for task in &task_contents {
        println!("{:?}", task)
    }
    let mut index = String::new();
    println!("Enter task number to be removed");
    io::stdin().read_line(&mut index).expect("Didnt receive input");
    let delete_index = index.trim().parse::<usize>().unwrap() - 1;
    let modified_elements = &task_contents[delete_index + 1..];
    let mut new_elements = Vec::new();
    for s in modified_elements {
        let mut iter = s.splitn(2, '.');
        let num_str = iter.next().unwrap();
        let rest = iter.next().unwrap();
        let new_num = num_str.parse::<u8>().unwrap() - 1;
        let new_s = format!("{}{}{}", new_num, ".", rest);
        new_elements.push(new_s);
    }
    let mut task_contents_copy = task_contents.clone();
    task_contents_copy.splice(delete_index..delete_index + new_elements.len(), new_elements);
    let completed_task = task_contents[delete_index].to_owned();
    write_removed_task(&task_contents_copy).unwrap();
    write_completed_tasks(&completed_task).unwrap();
    String::from("Task removed successfully")
}

fn write_completed_tasks(task_content: &str) -> std::io::Result<()> {
    // Modify the first two characters of the string
    let mut content = task_content.to_owned();
    content.replace_range(0..3, "");

    // Write the modified string to the file
    let mut file = File::create("completed.txt")?;
    file.write_all(content.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

fn write_removed_task(task_contents_copy: &Vec<String>) -> std::io::Result<()>{
    let mut file = File::create("tasks.txt")?;
    for task in task_contents_copy {
        file.write_all(task.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

fn show_help() -> String {
    let actions = vec!["add - to add new task to do list", "remove - to drop a task from to do list", "complete - to mark a task as complete", "summary - for overview of all tasks"];

    let mut help = String::new();
    for action in actions {
        help.push_str(action);
        help.push('\n');
    }

    help
}

fn show_summary() -> String {
    let open_tasks = read_task();
    let completed_tasks = read_completed_task();
    
    let mut summary = Vec::new();
    summary.push("Total Pending Tasks: ".to_owned() + &open_tasks.len().to_string()+"\n");
    summary.extend(open_tasks.iter().cloned());
    summary.push("\n".to_owned());
    summary.push("No of Completed Tasks: ".to_owned() + &completed_tasks.len().to_string()+"\n");
    summary.extend(completed_tasks.iter().cloned());

    let mut help = String::new();
    for action in summary {
        help.push_str(&action);
        help.push('\n');
    }
    help
    
}