use std::process::Command;
use std::{env, fs, str};
use std::path::Path;

fn more_recent_than(srcs: &Vec<String>, dst: &str) -> std::io::Result<bool> {
    match fs::metadata(dst) {
        Ok(metadata_dst) => {
            let time_mod_dst = metadata_dst.modified()?;
            for src in srcs {
                let time_mod_src = fs::metadata(src)?.modified()?;

                if time_mod_src > time_mod_dst {
                    return Ok(true)
                }
            } 
            return Ok(false)
        }
        _ => Ok(true)
    }
}

fn main() {
    let ext;
    let compiler;
    let cublas_flag;
    let xcompiler_flag;
    let dir;
    if Command::new("nvcc").status().is_ok() {
        ext = "cu";
        compiler = "nvcc";
        cublas_flag = "-lcublas";
        xcompiler_flag = "-Xcompiler";
        dir = "src/gpu";
        println!("cargo:rustc-link-lib=dylib=cublas");
        println!("cargo:rustc-link-lib=dylib=cudart");
    } else {
        ext = "cpp";
        compiler = "gcc";
        cublas_flag = "";
        xcompiler_flag = "";
        dir = Path::new("src").to_str().unwrap();
    };
    let c_names = vec!["matrix", "ops", "util"];

    let out_dir = env::var("OUT_DIR").expect("WTF 1");

    /////////////////
    Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/hello.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libmatrix.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    Command::new("g++").args(&["src/matrix.cpp", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/matrix.o", out_dir))
                       .status().expect("matrix.cpp test failed.");
    Command::new("ar").args(&["crus", "libmatrix.a", "matrix.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    Command::new("g++").args(&["src/ops.cpp", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/matrix.o", out_dir))
                       .status().expect("ops.cpp test failed.");
    Command::new("ar").args(&["crus", "libmatrix.a", "matrix.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    Command::new("g++").args(&["src/util.cpp", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/matrix.o", out_dir))
                       .status().expect("util.cpp test failed.");
    Command::new("ar").args(&["crus", "libmatrix.a", "matrix.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    //////////////

    //let get_out_name = |name| format!("{}/{}.o", out_dir, name);


    //for i in 0..c_names.len() {
        //let src_str = format!("./{}/{}.{}", dir, c_names[i], ext);
        //let src_name = Path::new(&src_str).to_str().unwrap();
        //let out_name = get_out_name(c_names[i]);

        //if more_recent_than(&vec![String::from(src_name)], &out_name).expect("WTF 2") {
            //let output = Command::new("find")
                            //.arg(".")
                            //.arg("-name")
                            //.arg(format!("{}.{}", c_names[i], ext)).output().expect("WTF 2.5");
            //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            //assert!(Command::new(compiler)
                //.arg(str::from_utf8(&output.stdout).unwrap())
                //.args(&["-c", xcompiler_flag, "-fPIC", cublas_flag, "-o"]) 
                //.arg(&out_name)
                //.status().expect("WTF 3").success(), "{} {} failed", compiler, src_name);
        //}
    //}

    //let out_files: Vec<String> = c_names.into_iter().map(get_out_name).collect();

    //if more_recent_than(&out_files, "libmatrix.a").expect("WTF 4") {

        //assert!(Command::new("rm")
            //.args(&["-f", "libmatrix.a"]) 
            //.current_dir(&Path::new(&out_dir)) 
            //.status().expect("WTF 5").success(), "rm failed");


        //assert!(Command::new("ar")
            //.args(&["crus", "libmatrix.a"])
            //.args(&out_files)
            //.current_dir(&Path::new(&out_dir)) 
            //.status().expect("WTF 6").success(), "ar failed");
    //}

    println!("cargo:rustc-link-search=native={}", out_dir);
    //if let Some(paths) = env::var_os("LD_LIBRARY_PATH") {
        //for path in env::split_paths(&paths) {
            //println!("cargo:rustc-link-search=native={}", path.display());
        //}
    //}
    println!("cargo:rustc-link-lib=static=matrix");
}
