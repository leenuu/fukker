use blake3::Hasher;
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;
use std::process::{self, Command, Stdio, Child};
use std::env;

#[derive(Deserialize)]
struct Config {
    watch_dir: Vec<String>,
    watch_ext: Vec<String>,
    working_dir: String,
    cmd : String,
    server_port: Vec<u16>,
    refresh_interval: u64,
    watch_buffer_size: usize
}

fn main() -> Result<(), Box<dyn Error>> {
    // JSON 파일에서 디렉토리 위치 파싱
    let config = read_config("config.json")?;
    let watch_dir_paths = config.watch_dir;
    let watch_exts = config.watch_ext;
    let working_dir_path= config.working_dir;
    let cmd = config.cmd;
    let server_ports = config.server_port;
    let refresh_interval = config.refresh_interval;
    let watch_buffer_size = config.watch_buffer_size;
    
    // 현재 작업 디렉토리를 변경
    env::set_current_dir(&working_dir_path)?;

    let mut old_hash_files_map = hash_directory(&watch_dir_paths, &watch_exts, &watch_buffer_size)?;
    let mut watch_hash_files_map: HashMap<String, String> = HashMap::new();

    let mut child = run_process(&cmd)?;

    loop {
        let mut changed = false;
        let new_hash_files_map = hash_directory(&watch_dir_paths,&watch_exts, &watch_buffer_size)?;
        for (key, value) in &new_hash_files_map {
            if old_hash_files_map.contains_key(key) {
                if old_hash_files_map.get(key).unwrap() != value {
                    println!("파일 변경: {:?}", key);
                    changed = true;
                    break;
                }
            } else {
                println!("새 파일: {:?}", key);
                changed = true;
                break;
            }
        }
        for (key, _) in &old_hash_files_map {
            if !new_hash_files_map.contains_key(key) {
                println!("파일 삭제: {:?}", key);
                changed = true;
                break;
            }
        }

        if changed {
            println!("파일 변경이 감지되었습니다. 프로세스를 재시작합니다.");
            process::Command::new("kill")
            .arg("-9")
            .arg(child.id().to_string())
            .spawn()?;

            let mut pids: Vec<u32> = Vec::new();
            for port in &server_ports {
                let pid = find_pid_by_port(port)?;
                if pid != 0 {
                    pids.push(pid);
                }
            }

            kill_pids(&pids)?;
            
            child = run_process(&cmd)?;
            changed = false;
            std::thread::sleep(std::time::Duration::from_millis(5000)); 
        }
        old_hash_files_map = new_hash_files_map;
        watch_hash_files_map.clear();
        std::thread::sleep(std::time::Duration::from_millis(refresh_interval));
    }   

    Ok(())
}

// JSON 파일을 읽고 Config 구조체로 변환
fn read_config(filename: &str) -> serde_json::Result<Config> {
    let file = File::open(filename).expect("파일을 열 수 없습니다.");
    let config: Config = serde_json::from_reader(file)?;
    Ok(config)
}

// 디렉토리 내 모든 파일의 해시를 계산
fn hash_directory(directory: &Vec<String>, watch_ext: &Vec<String>, buffer_size: &usize) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut hash_files_map: HashMap<String, String> = HashMap::new();

    for dir in directory {
        for entry in WalkDir::new(dir) {
            let entry = entry?;
            if let Some(extension) = entry.path().extension() {
                let ext_str = extension.to_str().unwrap().to_lowercase();
                // watch_ext에 해당하는 확장자만 처리
                if watch_ext.contains(&ext_str) {
                    let file_path = entry.path().to_str().unwrap();
                    let hash = hash_file(file_path, buffer_size)?;
                    let path = format!("{}", file_path);
                    hash_files_map.insert(path, hash);
                }
            }
        }
    }
    Ok(hash_files_map)
}


// 파일의 해시를 계산
fn hash_file(filepath: &str, buffer_size: &usize) -> Result<String, Box<dyn Error>>{
    let mut hasher = Hasher::new();
    let mut file = File::open(filepath).expect( "파일을 열 수 없습니다.");
    let mut buffer = vec![0; *buffer_size];
    
    loop {
        let bytes_read = file.read(&mut buffer).expect("파일을 읽을 수 없습니다.");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

fn run_process(command: &str) -> Result<Child, Box<dyn Error>> {
    let mut cmd_parts = command.split_whitespace();
    let program = cmd_parts.next().ok_or("명령어가 비어 있습니다")?;
    let args: Vec<&str> = cmd_parts.collect();

    let child = Command::new(program)
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    Ok(child)
}

fn find_pid_by_port(port: &u16) -> Result<u32, Box<dyn Error>> {
    let mut pid = 0;
    if let Ok(listeners) = listeners::get_all() {
        for l in listeners {
            if *port == l.socket.port() {
               pid = l.process.pid;
            }
        }
    }
    Ok(pid)
}

fn kill_pids(pids: &Vec<u32>) -> Result<(), Box<dyn Error>> {
    for pid in pids {
        process::Command::new("kill")
            .arg("-9")
            .arg(&pid.to_string())
            .spawn()?;
    }
    Ok(())
}
