use std::process::exit;
#[cfg(target_os = "windows")]
use winapi::um::{processthreadsapi::TerminateProcess, handleapi::CloseHandle, winnt::PROCESS_TERMINATE,tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, TH32CS_SNAPPROCESS, PROCESSENTRY32}};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use libc::{kill, SIGKILL, SIGTERM};
use std::{env, process::{Command, Stdio}, io::{BufReader, BufRead}};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: process_killer <process_name1> [process_name2] [process_name3] ...");
        exit(1);
    }

    // 获取所有进程名（跳过第一个参数，因为它是程序名称）
    let process_names = &args[1..];

    // 遍历每个进程名
    for process_name in process_names {
        println!("Looking for processes with name: {}", process_name);
        
        let pids = match find_pids(process_name) {
            Ok(pids) => pids,
            Err(err) => {
                eprintln!("Error finding PIDs for process {}: {}", process_name, err);
                continue; // 继续处理下一个进程名
            }
        };

        if pids.is_empty() {
            eprintln!("No process found with name: {}", process_name);
            continue;
        }

        for pid in pids {
            if kill_process(pid).is_err() {
                eprintln!("Failed to kill process {} (PID: {})", process_name, pid);
            } else {
                println!("Process {} (PID: {}) killed successfully.", process_name, pid);
            }
        }
    }

}

#[cfg(target_os = "windows")]
fn find_pids(process_name: &str) -> Result<Vec<u32>, String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let mut pids = Vec::new();

    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if snapshot == winapi::um::handleapi::INVALID_HANDLE_VALUE {
        return Err("Failed to create snapshot".to_string());
    }

    let mut entry: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;


    if unsafe { Process32First(snapshot, &mut entry) } == 0 {
        unsafe { CloseHandle(snapshot) };
        return Err("Failed to get first process".to_string());
    }

    loop {
        let mut name_vec = Vec::new();
        for &c in entry.szExeFile.iter(){
             if c == 0{
                 break;
             }
            name_vec.push(c as u8);
        }
        let process_exe_name = String::from_utf8_lossy(&name_vec);

        if process_exe_name.to_lowercase() == process_name.to_lowercase(){
            pids.push(entry.th32ProcessID);
        }


        if unsafe { Process32Next(snapshot, &mut entry) } == 0 {
            break;
        }
    }
    unsafe { CloseHandle(snapshot) };

    Ok(pids)
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn find_pids(process_name: &str) -> Result<Vec<u32>, String> {
    let output = Command::new("ps")
        .args(&["-ax", "-o", "pid,comm"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute ps command: {}", e))?;
    if !output.status.success() {
         return Err(format!("ps command failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let reader = BufReader::new(output.stdout.as_slice());
    let mut pids = Vec::new();

    for line in reader.lines(){
        let line = line.map_err(|e| format!("Failed to read line from ps output: {}", e))?;
         let parts: Vec<&str> = line.trim().split_whitespace().collect();
         if parts.len() < 2{
              continue;
         }

         let pid_str = parts[0];
         let current_process_name = parts[1];

          if current_process_name.to_lowercase() == process_name.to_lowercase() {
                let pid = pid_str.parse::<u32>().map_err(|e|format!("Invalid pid {} from ps output: {}",pid_str, e))?;
                pids.push(pid);
          }

    }
    Ok(pids)
}


#[cfg(target_os = "windows")]
fn kill_process(pid: u32) -> Result<(), ()>{
    use winapi::um::processthreadsapi::OpenProcess;
    let handle = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
    if handle.is_null() {
        return Err(());
    }
    let result = unsafe { TerminateProcess(handle, 1) };
    unsafe { CloseHandle(handle) };
    if result == 0{
      return Err(());
    }
    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn kill_process(pid: u32) -> Result<(), ()>{
    let result = unsafe { kill(pid as i32, SIGKILL) };
    if result != 0{
        return Err(());
    }
    Ok(())
}