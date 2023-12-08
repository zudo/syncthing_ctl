use clap::Parser;
use clap::Subcommand;
use lazy_static::lazy_static;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::processthreadsapi::TerminateProcess;
use winapi::um::tlhelp32::CreateToolhelp32Snapshot;
use winapi::um::tlhelp32::Process32First;
use winapi::um::tlhelp32::Process32Next;
use winapi::um::tlhelp32::PROCESSENTRY32;
use winapi::um::tlhelp32::TH32CS_SNAPPROCESS;
use winapi::um::winbase::CREATE_NO_WINDOW;
lazy_static! {
    static ref STARTUP: PathBuf = {
        dirs::home_dir()
            .unwrap()
            .join("AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup")
            .join("syncthing.bat")
    };
}
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Start {
        #[clap(long, default_value = "C:\\syncthing\\syncthing.exe")]
        path: String,
        #[clap(last = true)]
        args: Vec<String>,
    },
    Stop,
    Enable {
        #[clap(last = true)]
        args: Vec<String>,
    },
    Disable,
}
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start { path, args } => {
            let mut command = Command::new(path);
            command.args(args);
            command.creation_flags(CREATE_NO_WINDOW);
            let child = command.spawn().expect("Process failed to start");
            println!("Process started with PID: {}", child.id());
        }
        Commands::Stop => {
            for process in find_processes_by_name("syncthing.exe") {
                stop_process(process.th32ProcessID);
            }
        }
        Commands::Enable { args } => {
            let content = if args.is_empty() {
                format!("syncthing start")
            } else {
                format!("syncthing start -- {}", args.join(" "))
            };
            let mut file = File::create(&*STARTUP).expect("Failed to create .bat file");
            file.write_all(content.as_bytes())
                .expect("Failed to write to .bat file");
            println!("Startup enabled.");
        }
        Commands::Disable => {
            if STARTUP.exists() {
                fs::remove_file(&*STARTUP).expect("Failed to remove .bat file");
                println!("Startup disabled.");
            } else {
                println!("Startup script not found.");
            }
        }
    }
}
pub fn find_processes_by_name(name: &str) -> Vec<PROCESSENTRY32> {
    let name_cstr = CString::new(name).expect("Failed to convert process name to CString");
    let mut processes = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot != INVALID_HANDLE_VALUE {
            let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
            process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;
            if Process32First(snapshot, &mut process_entry) != 0 {
                loop {
                    let process_name = CStr::from_ptr(process_entry.szExeFile.as_ptr());
                    if process_name.to_string_lossy().to_lowercase()
                        == name_cstr.to_str().unwrap().to_lowercase()
                    {
                        processes.push(process_entry);
                    }
                    if Process32Next(snapshot, &mut process_entry) == 0 {
                        break;
                    }
                }
            }
            CloseHandle(snapshot);
        }
    }
    processes
}
pub fn stop_process(pid: u32) {
    unsafe {
        let process_handle = OpenProcess(0x0001, 0, pid);
        if process_handle != std::ptr::null_mut() {
            TerminateProcess(process_handle, 0);
            CloseHandle(process_handle);
        }
    }
    println!("Stopping process with PID: {}", pid);
}
