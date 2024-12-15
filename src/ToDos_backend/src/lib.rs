
use ic_cdk_macros::{query, update};
use candid::{CandidType, Deserialize};
use std::cell::RefCell;

// Define the Task struct
#[derive(CandidType, Deserialize, Clone, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

// In-memory state for tasks
thread_local! {
    static TASKS: RefCell<Vec<Task>> = RefCell::new(Vec::new());
}

// Add a task
#[ic_cdk_macros::update]
fn add_task(description: String) -> String {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let id = (tasks.len() as u32) + 1; // Generate unique ID
        tasks.push(Task {
            id,
            description: description.clone(),
            done: false,
        });
        format!("Task added with ID: {}", id)
    })
}

// List all tasks
#[query]
fn list_tasks() -> Vec<Task> {
    TASKS.with(|tasks| tasks.borrow().clone())
}

// Mark a task as done
#[update]
fn mark_task_done(id: u32) -> String {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
            task.done = true;
            return format!("Task {} marked as done", id);
        }
        format!("Task with ID {} not found", id)
    })
}

// Delete a task
#[update]
fn delete_task(id: u32) -> String {
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        if let Some(pos) = tasks.iter().position(|task| task.id == id) {
            tasks.remove(pos);
            return format!("Task {} deleted", id);
        }
        format!("Task with ID {} not found", id)
    })
}

