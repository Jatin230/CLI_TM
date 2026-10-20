use std::io;
#[derive(Debug)]
struct Task {
    title: String,
    description: Option<String>,
    done: bool,
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut title = String::new();
    println!("Enter the title of the task: ");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    loop {
        if title.trim().is_empty() {
            println!("Title cannot be empty. Please enter a valid title: ");
            title.clear();
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");
        } else {
            break;
        }
    }
    let mut description = String::new();
    println!("Enter the description of the task: ");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    tasks.push(Task {
        title: title.trim().to_string(),
        description: if description.trim().is_empty() {
            None
        } else {
            Some(description.trim().to_string())
        },
        done: false,
    });

    for (i, task) in tasks.iter().enumerate() {
        println!("Sno: {}", i + 1);
        println!("Title: {}", task.title);
        match &task.description {
            Some(desc) => println!("Description: {}", desc),
            None => println!("Description: None"),
        }
        let done = if task.done { "✅" } else { "❌" };
        println!("Done: {}", done);
    }
}
