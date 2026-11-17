use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // ایجاد یک فایل متنی با نام hello.txt در کنار فایل اجرایی
    let mut file = File::create("hello.txt")?;
    
    // نوشتن کلمه hello داخل فایل
    file.write_all(b"hello")?;
    
    println!("فایل با موفقیت ساخته شد و متن داخل آن نوشته شد!");
    Ok(())
}