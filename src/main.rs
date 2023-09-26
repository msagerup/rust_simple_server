use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DB {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl DB {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    // TASK FUNCTIONS
    fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }
    // Get task by id
    fn get(&self, id: &u64) -> Option<&Task> {
        self.tasks.get(id)
    }
    // Get all tasks
    fn get_all(&self, id: &u64) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    // delete task
    fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }
    // update task
    fn update(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    // USER FUNCTIONS
    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }

    // Search for username, with find method.
    fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // DB saving.
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file_content = fs::read_to_string("database.json")?;
        let db = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

fn main() {
    //   Connect db.
}
