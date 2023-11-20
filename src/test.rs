use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::winbase::INFINITE;
use winapi::um::winnt::{MEM_COMMIT, PAGE_READWRITE, PROCESS_ALL_ACCESS};

fn main() {
    unsafe {
        // 1. 获取目标进程的句柄
        let process_handle = OpenProcess(PROCESS_ALL_ACCESS, 0, 120003);

        // 2. 在目标进程中分配内存
        let buffer: Vec<u8> = vec![
            0x67, 0x00, 0x00, 0x00, // push 00000067
            0x68, 0x48, 0xBD, 0x81, 0x00, // push 0081bd48
            0x6A, 0x2C, // push 202c
            0xE8, 0x40, 0x64, 0x59, 0x00, // call 00596440
            0x83, 0xC4, 0x0C, // add esp,0c
        ];

        let code_size = buffer.len();
        let code_address = VirtualAllocEx(
            process_handle,
            std::ptr::null_mut(),
            code_size,
            MEM_COMMIT,
            PAGE_READWRITE,
        );

        // 3. 写入汇编代码到目标进程的内存中
        let bytes_written = 0;
        WriteProcessMemory(
            process_handle,
            code_address,
            buffer.as_ptr() as *const _,
            code_size,
            &bytes_written,
        );

        // 4. 创建远程线程
        CreateRemoteThread(
            process_handle,
            std::ptr::null_mut(),
            0,
            Some(std::mem::transmute(code_address)),
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
        );

        // 这里可能需要等待一段时间，以确保线程执行完毕
        // WaitForSingleObject等待远程线程的完成
        // WaitForSingleObject(hRemoteThread, INFINITE);
    }
}
