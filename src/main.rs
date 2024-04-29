use std::{env, str::FromStr};
use clap::Parser;
use netcorehost::{*, pdcstring::PdCString, hostfxr::Hostfxr};
use climeta::{*, schema::TypeDef};

/// Test Program for running NetCore assemblies
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to load the assembly from
    path: String,
}

thread_local!{
    static DOTNETRUNTIME: Hostfxr = nethost::load_hostfxr().unwrap();
}

fn main() {
    // let args = Args::parse();

    let path = env::current_exe().expect("Current executable path to be valid").parent().expect("Current executable to have a parent").join("csharp.dll");

    let cache = Cache::new();

    cache.insert(Database::from_file(&path).expect("Loading IL to work"));


    for db in &cache {
        for typedef in db.table::<TypeDef>() {



            println!("Found {:?} \"{}\"", typedef.type_category().unwrap(), typedef.type_name().unwrap());

            for method in typedef.method_list().unwrap() {
                let flags = method.flags().unwrap();
                let signature = method.signature().unwrap();
                let args = method
                    .param_list().unwrap()
                    .filter_map(|a| a.name().and_then(|f| Ok(f.to_string())).ok())
                    .reduce(|f, a| format!("{}, {}", f, a))
                    .or(Some("".to_string())).unwrap();

                println!("\t {} {:?} {:?} {}({})", if flags.static_() { "static" } else { "" }, flags.access(), signature.return_type().kind(), method.name().unwrap(), args);
            }
        }
    }

    let path = PdCString::from_str(&path.to_str().unwrap()).expect("path to be utf16 compatible");

    let context = DOTNETRUNTIME.with(|runtime| runtime.initialize_for_dotnet_command_line(path).unwrap());
    let result = context.run_app().value();
    
    println!("Result was {}", result);
}
