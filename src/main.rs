use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
extern crate reqwest;
extern crate getopts;
use getopts::Options;
use std::env;

static URL: &str = "http://127.0.0.1:8080/"; // URL веб-сервера по умолчанию
static BIND: &str = "127.0.0.1:8000"; // хост:порт для подключений клиентов по умолчанию
static VALID_TIME: u64 = 20;

struct CacheObject
{
    response:String,
    timestamp:SystemTime,
    url:String
}

fn fetch(co:&mut CacheObject)
{
    let url = format!("{}", co.url);
    co.response = reqwest::get(&url).unwrap().text().unwrap(); 
    co.timestamp = SystemTime::now();
}

fn index(co: web::Data<Mutex<CacheObject>>, _req: HttpRequest) -> HttpResponse
{
    {
        let mut co = &mut *(co.lock().unwrap());
        if SystemTime::now().duration_since(co.timestamp).unwrap().as_secs() >= VALID_TIME
        {
            fetch(&mut co);
        }
    }
    let co = &*(co.lock().unwrap());
    HttpResponse::Ok().header("Cache-Control", format!("only-if-cached,max-age={}", VALID_TIME - co.timestamp.elapsed().unwrap().as_secs())).body(co.response.clone())
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("{} -U <url> -B <local address>", program);
    print!("{}", opts.usage(&brief));
}

fn main()
{
    let mut localbind: String = BIND.to_string();
    let mut url: String = URL.to_string();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("U", "url", "URL веб-сервера (по умолчанию http://127.0.0.1:8000)", "URL");
    opts.optopt("B", "bind", "хост:порт для подключений клиентов (по умолчанию 127.0.0.1:8888)", "BIND");
    opts.optflag("h", "help", "справка");
    let matches = match opts.parse(&args[1..]) 
    {
        Ok(m) => { m }
        Err(_) => { print_usage(&program, opts); return (); }
    };
    if matches.opt_present("h") 
    {
        print_usage(&program, opts);
        return ();
    }
    if matches.opt_present("U")
    {
        url = matches.opt_str("U").unwrap().clone();
    }
    if matches.opt_present("B")
    {
        localbind = matches.opt_str("B").unwrap().clone();
    }
    println!("Хост: {}", localbind);
    println!("URL: {}", url);
    match reqwest::get(&url)
    {
        Ok(_) => {},
        Err(e) => {println!("Неправильный URL: {}", e); return ();}
    }
    let cache = web::Data::new(Mutex::new(CacheObject {response:"".to_string(), timestamp:UNIX_EPOCH, url:url}));
    let srv = HttpServer::new(move || {
        App::new()
            .register_data(cache.clone())
            .service(web::resource("/").to(index))
    });
    match srv.bind(localbind) 
    {
        Ok(srv) => {srv.run().unwrap();},
        Err(e) => {println!("Ошибка хоста: {}", e);}
    }
}
