# weekly66
Software Development

---
- Rust
---

# Rust

<https://www.rust-lang.org/>

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
echo "export PATH=$PATH:~/.cargo/bin" >> ~/.zshrc && . ~/.zshrc
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ cargo --version
cargo 1.73.0 (9c4383fb5 2023-08-26)
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66/devrust]
└─$ cargo new simple01  
     Created binary (application) `simple01` package

┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple01]
└─$ cargo run         
Hello, world!	 

┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple01]
└─$ cargo test
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

- Ownership in Rust
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.

  - Slice
```rust
    let s = String::from("hello world");
    let hello = &s[0..5];    // portion of the string
    let world = &s[6..11];   // portion of the string
```

Chapter 6 - enum  https://doc.rust-lang.org/book/ch06-00-enums.html
enums Message

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));
m.call();
```

```rust
enum Option<T> {
    None,
    Some(T),
}
```


Chapter 6 Pattern Matching :: https://doc.rust-lang.org/book/ch06-02-match.html


```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

```rust

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```

Chapter 7 : packages and crates : https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html



```sh
cargo new messages

cd messages

cargo run  
```

```sh
src/main.rs
src/lib.rs
src/bin
```

```sh
src/garden.rs
src/garden/mod.rs
```

```rust
use
mod
pub mod
```

```sh
cargo new restaurant --lib
```

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

Chapter 8  : Collections : https://doc.rust-lang.org/book/ch08-00-common-collections.html

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
	
	let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// format!() macro uses references
let s = format!("{s1}-{s2}-{s3}");

// or
let s = s1 + "-" + &s2 + "-" + &s3;
```

```rust
let hello = "Здравствуйте"; // String Russian

for c in "Зд".chars() {
    println!("{c}");
}
```

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
	
    for (key, value) in &scores {
        println!("{key}: {value}");
    }	

    scores.entry(String::from("Blue")).or_insert(50);
```

```rust

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

Chapter 9 : Error Handling  : https://doc.rust-lang.org/book/ch09-00-error-handling.html


```rust
fn main() {
    panic!("crash and burn");
}
```

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

- Clousures
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

```rust
//Propagating Errors: the ? Operator
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```


```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

```rust
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
```

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

```

Chapter 10 : Generics : https://doc.rust-lang.org/book/ch10-00-generics.html

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Chapter 10 : Traits : https://doc.rust-lang.org/book/ch10-02-traits.html



```rust
pub trait Summary {
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// different type
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// same type
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Chapter 11 : test : https://doc.rust-lang.org/book/ch11-01-writing-tests.html

```rust
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```


```sh
# run test in parallel default
cargo test 

# run test in serie
cargo test -- --test-threads=1

cargo test -- --show-output
```


- web server, axum framework

<https://docs.rs/axum/latest/axum/>


main.rs
```rust
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

```

Cargo.toml
```
[package]
name = "simple02"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum  =  "0.6"
hyper = { version = "1.0.0-rc.4", features = ["full"] }
tokio = { version = "1", features = ["full"] }
tower = { version = "0.4", features = ["full"] }
```

https://docs.rs/axum/latest/axum/

```sh
┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple02]
└─$ cargo add tokio --features macros,rt-multi-thread
    Updating crates.io index
      Adding tokio v1 to dependencies.
             Features as of v1.0.1:
             + bytes
             + fs
             + full
             + io-std
             + io-util
             + libc
             + macros
             + memchr
             + net
             + num_cpus
             + once_cell
             + parking_lot
             + process
             + rt
             + rt-multi-thread
             + signal
             + signal-hook-registry
             + sync
             + time
             + tokio-macros
             - mio
             - test-util
             - tracing
             - winapi
```

Cargo.toml
```
[dependencies]
tokio = { version = "1", features = ["full", "macros", "rt-multi-thread"] }
```

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

```rust
#[cfg(test)]
mod tests {

    #[test]
    fn internal() {
        assert_eq!(4, 2 + 2);
    }
}
```


----

```rust
use ::axum::Router;
use ::axum::routing::get;

use ::axum_test::TestServer;

async fn get_ping() -> &'static str {
    "pong!"
}

    
#[tokio::test]
async fn it_should_get() {
    // Build an application with a route.
    let app = Router::new()
        .route("/ping", get(get_ping));

    // Run the application for testing.
    let server = TestServer::new(app).unwrap();

    // Get the request.
    let response = server
        .get("/ping")
        .await;

    assert_eq!(response.text(), "pong!");
}

```

```sh
cargo add axum
cargo add tokio --features macros,rt-multi-thread
cargo add axum-test 
```

```
[package]
name = "simple05"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
axum-test = "13.1.1"
tokio = { version = "1.34.0", features = ["macros", "rt-multi-thread"] }
```


```sh
┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple05]
└─$ cargo test                                       
   Compiling tokio-macros v2.2.0
   Compiling num_cpus v1.16.0
   Compiling tokio v1.34.0
   Compiling hyper v0.14.27
   Compiling tower v0.4.13
   Compiling axum v0.6.20
   Compiling axum-test v13.1.1
   Compiling simple05 v0.1.0 (/home/kali/projects/weekly66/devrust/simple05)
    Finished test [unoptimized + debuginfo] target(s) in 27.36s
     Running unittests src/main.rs (target/debug/deps/simple05-3de4e7e867e66377)

running 1 test
test it_should_get ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```



---

```rust
use ::axum_test::TestServer;

///////////////

use axum::{
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
      .route("/", get(|| async { "main Working..!" }))
      .route("/ping", get(get_ping));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


// main test    
#[tokio::test]
async fn it_should_work() {
    // Build an application with a route.
    let app = Router::new()
    .route("/", get(|| async { "main Working..!" }));
    // Run the application for testing.
    let server = TestServer::new(app).unwrap();
    // Get the request.
    let response = server
        .get("/")
        .await;
    assert_eq!(response.text(), "main Working..!");
}


///////////////

// get 
async fn get_ping() -> &'static str {
    "pong!"
}

// get test    
#[tokio::test]
async fn it_should_get() {
    // Build an application with a route.
    let app = Router::new()
        .route("/ping", get(get_ping));

    // Run the application for testing.
    let server = TestServer::new(app).unwrap();

    // Get the request.
    let response = server
        .get("/ping")
        .await;

    assert_eq!(response.text(), "pong!");
}

```

```
[package]
name = "simple05"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
tokio = { version = "1.34.0", features = ["macros", "rt-multi-thread"] }
axum-test = "13.1.1"

```

```sh
┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple05]
└─$ cargo test
   Compiling simple05 v0.1.0 (/home/kali/projects/weekly66/devrust/simple05)
    Finished test [unoptimized + debuginfo] target(s) in 3.91s
     Running unittests src/main.rs (target/debug/deps/simple05-3de4e7e867e66377)

running 2 tests
test it_should_get ... ok
test it_should_work ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


---



```rust
//use ::axum_test::TestServer;

///////////////

use axum::{
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
      .route("/", get(|| async { "main Working..!" }))
      .route("/ping", get(get_ping));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[cfg(test)]
mod test_main{
  use ::axum_test::TestServer;
  use axum::{
    routing::get,
    Router,
  };

  // main test    
  #[tokio::test]
  async fn it_should_work() {
    // Build an application with a route.
    let app = Router::new()
    .route("/", get(|| async { "main Working..!" }));
    // Run the application for testing.
    let server = TestServer::new(app).unwrap();
    // Get the request.
    let response = server
        .get("/")
        .await;
    assert_eq!(response.text(), "main Working..!");
  }
}

// get 
async fn get_ping() -> &'static str {
    "pong!"
}

#[cfg(test)]
mod test_ping{
    use ::axum_test::TestServer;
    use axum::{
        routing::get,
        Router,
      };    
    use super::{get_ping};
  // get test 
  #[tokio::test]
  async fn it_should_get() {
    // Build an application with a route.
    let app = Router::new()
        .route("/ping", get(get_ping));
    // Run the application for testing.
    let server = TestServer::new(app).unwrap();
    // Get the request.
    let response = server
        .get("/ping")
        .await;
    assert_eq!(response.text(), "pong!");
  }
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple05]
└─$ cargo test
   Compiling simple05 v0.1.0 (/home/kali/projects/weekly66/devrust/simple05)
    Finished test [unoptimized + debuginfo] target(s) in 4.50s
     Running unittests src/main.rs (target/debug/deps/simple05-3de4e7e867e66377)

running 2 tests
test test_main::it_should_work ... ok
test test_ping::it_should_get ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple05]
└─$ cargo run 
   Compiling simple05 v0.1.0 (/home/kali/projects/weekly66/devrust/simple05)
    Finished dev [unoptimized + debuginfo] target(s) in 4.77s
     Running `target/debug/simple05`
```


```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:3000/ 
main Working..!                                                                                                                      
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:3000/ping
pong! 
```


---

```sh
cargo new cli
cargo new srv
```

```
┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple06/cli]
└─$ cargo add axum

```

```rust
use axum::response::Html;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello", 
        get(|| async { Html("<h1>Working Rust!</h1>") } )
    );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_hello.into_make_service())
  .await
  .unwrap();
}

```

```
use of undeclared crate or module `tokio`

┌──(kali㉿kali)-[~/projects/weekly66/devrust/simple06/cli]
└─$ cargo add tokio --features macros,rt-multi-thread
```

```sh
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
tokio = { version = "1.34.0", features = ["macros", "rt-multi-thread"] }
```

```sh
cargo run

curl http://localhost:3000/hello
```


---

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:3000/hello
<h1>Working Rust!</h1>   
```

```rust
use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello", 
        get( handler_hello )
    );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_hello.into_make_service())
  .await
  .unwrap();
}

async fn handler_hello() -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler","HANDLER");
  Html("<h1>Working Rust!</h1>")
}
```

---

```sh
┌──(kali㉿kali)-[~/…/weekly66/devrust/simple06/cli]
└─$ cargo add serde     

┌──(kali㉿kali)-[~/…/weekly66/devrust/simple06/cli]
└─$ cargo add serde_json
```

<https://serde.rs/derive.html>

```sh
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
serde = { version = "1.0.192", features = ["derive"]  }
serde_json = "1.0.108"
tokio = { version = "1.34.0", features = ["macros", "rt-multi-thread"] }

```

```rust
use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Query;
use std::net::SocketAddr;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello", 
        get( handler_hello )
    );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_hello.into_make_service())
  .await
  .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler {params:?}","HANDLER");
  let name = params.name.as_deref().unwrap_or("Default");
  Html(format!("<h1>Working Rust! {name}!</h1>"))
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:3000/hello?name=Maximiliano
<h1>Working Rust! Maximiliano!</h1> 
```

---


```rust
use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Query;
use axum::extract::Path;
use std::net::SocketAddr;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
    .route( "/hello", get( handler_hello ) )
    .route( "/hello2/:name", get( handler_hello2 ) );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_hello.into_make_service())
  .await
  .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// /hello?name=Maximiliano
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler {params:?}","HANDLER");
  let name = params.name.as_deref().unwrap_or("Default");
  Html(format!("<h1>Working Rust! {name}!</h1>"))
}

// /hello/CarloAcuti
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(" ->> {:<12} - hello_handler2 {name:?}","HANDLER");
    Html(format!("<h1>Working Rust! {name}!</h1>"))
}
```


```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:3000/hello2/CarloAcutis
<h1>Working Rust! CarloAcutis!</h1>   
```


---


```sh
┌──(kali㉿kali)-[~/…/weekly66/devrust/simple06/cli]
└─$ cargo add tower-http --features fs
```

```rust
use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Query;
use axum::extract::Path;
use std::net::SocketAddr;
use serde::Deserialize;
use tower_http::services::ServeDir;
use axum::routing::get_service;

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
    .merge( routes_hello() )
    .fallback_service( routes_static() );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_all.into_make_service())
  .await
  .unwrap();
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
  Router::new()
    .route( "/hello", get( handler_hello ) )
    .route( "/hello2/:name", get( handler_hello2 ) )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// /hello?name=Maximiliano
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler {params:?}","HANDLER");
  let name = params.name.as_deref().unwrap_or("Default");
  Html(format!("<h1>Working Rust! {name}!</h1>"))
}

// /hello/CarloAcuti
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(" ->> {:<12} - hello_handler2 {name:?}","HANDLER");

    Html(format!("<h1>Working Rust! {name}!</h1>"))
}
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:3000/Cargo.toml        
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
serde = { version = "1.0.192", features = ["derive"]  }
serde_json = "1.0.108"
tokio = { version = "1.34.0", features = ["macros", "rt-multi-thread"] }
tower-http = { version = "0.4.4", features = ["fs"] }
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:3000/src/main.rs
use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Query;
use axum::extract::Path;
use std::net::SocketAddr;
use serde::Deserialize;
use tower_http::services::ServeDir;
use axum::routing::get_service;

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
    .merge( routes_hello() )
    .fallback_service( routes_static() );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_all.into_make_service())
  .await
  .unwrap();
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
  Router::new()
    .route( "/hello", get( handler_hello ) )
    .route( "/hello2/:name", get( handler_hello2 ) )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// /hello?name=Maximiliano
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler {params:?}","HANDLER");
  let name = params.name.as_deref().unwrap_or("Default");
  Html(format!("<h1>Working Rust! {name}!</h1>"))
}

// /hello/CarloAcuti
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(" ->> {:<12} - hello_handler2 {name:?}","HANDLER");

    Html(format!("<h1>Working Rust! {name}!</h1>"))
}
```



---



```rust
pub use self::error::{Error, Result};

use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Query;
use axum::extract::Path;
use std::net::SocketAddr;
use serde::Deserialize;
use tower_http::services::ServeDir;
use axum::routing::get_service;

mod error;


#[tokio::main]
async fn main() {
  let routes_all = Router::new()
    .merge( routes_hello() )
    .fallback_service( routes_static() );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_all.into_make_service())
  .await
  .unwrap();
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
  Router::new()
    .route( "/hello", get( handler_hello ) )
    .route( "/hello2/:name", get( handler_hello2 ) )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// /hello?name=Maximiliano
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler {params:?}","HANDLER");
  let name = params.name.as_deref().unwrap_or("Default");
  Html(format!("<h1>Working Rust! {name}!</h1>"))
}

// /hello/CarloAcuti
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(" ->> {:<12} - hello_handler2 {name:?}","HANDLER");

    Html(format!("<h1>Working Rust! {name}!</h1>"))
}
```

```rust
// cli/src/error/mod.rs
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - {self:?}","INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}s
```

```sh
┌──(kali㉿kali)-[~/…/weekly66/devrust/simple06/cli]
└─$ cargo run
   Compiling cli v0.1.0 (/home/kali/projects/weekly66/devrust/simple06/cli)
    Finished dev [unoptimized + debuginfo] target(s) in 4.93s
     Running `target/debug/cli`
Listening: 127.0.0.1:3000
```


---


```rust
pub use self::error::{Error, Result};

use axum::response::Html;
use axum::response::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use axum::extract::Query;
use axum::extract::Path;
use std::net::SocketAddr;
use serde::Deserialize;
use tower_http::services::ServeDir;
use axum::routing::get_service;

mod error;
mod web;

#[tokio::main]
async fn main() {
  let routes_all = Router::new()
  .merge( routes_hello() )
  .merge( web::routes_login::routes() )
  .fallback_service( routes_static() );

  let addr = SocketAddr::from( ([127,0,0,1], 3000) );
  println!("Listening: {addr}\n");
  axum::Server::bind(&addr)
  .serve(routes_all.into_make_service())
  .await
  .unwrap();
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
  Router::new()
    .route( "/hello", get( handler_hello ) )
    .route( "/hello2/:name", get( handler_hello2 ) )
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}

// /hello?name=Maximiliano
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!(" ->> {:<12} - hello_handler {params:?}","HANDLER");
  let name = params.name.as_deref().unwrap_or("Default");
  Html(format!("<h1>Working Rust! {name}!</h1>"))
}

// /hello/CarloAcuti
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(" ->> {:<12} - hello_handler2 {name:?}","HANDLER");

    Html(format!("<h1>Working Rust! {name}!</h1>"))
}
```

```rust
// cli/src/error/mod.rs
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    LoginFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - {self:?}","INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
```

```rust
// cli/src/web/mod.rs
pub mod routes_login;
```

```rust
// cli/src/web/routes_login.rs
use crate::{Result, Error};
use serde::Deserialize;
use axum::Json;
use serde_json::Value;
use serde_json::json;
use axum::Router;
use axum::routing::post;

// export interface routes
pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}


async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>>{
  println!("--> {:<12} - api_login","HANDLER");

  //TODO: Implement db

  if payload.username != "demo" || payload.pwd != "demo" {
    return Err(Error::LoginFail);
  }

  //TODO: Set cookies

  let body = Json(json!({
    "result":{
        "success": true,
    }
  }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

```


```sh
[package]
name = "cli"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = "0.6.20"
serde = { version = "1.0.192", features = ["derive"]  }
serde_json = "1.0.108"
tokio = { version = "1.34.0", features = ["macros", "rt-multi-thread"] }
tower-http = { version = "0.4.4", features = ["fs"] }

```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl -v http://localhost:3000/api/login  -d '{ "username":"demo", "pwd":"demo" }' -H 'Content-Type: application/json'
*   Trying [::1]:3000...
* connect to ::1 port 3000 failed: Connection refused
*   Trying 127.0.0.1:3000...
* Connected to localhost (127.0.0.1) port 3000
> POST /api/login HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/8.3.0
> Accept: */*
> Content-Type: application/json
> Content-Length: 35
> 
< HTTP/1.1 200 OK
< content-type: application/json
< content-length: 27
< date: Tue, 14 Nov 2023 14:47:03 GMT
< 
* Connection #0 to host localhost left intact
{"result":{"success":true}}    
```


---






---

----

TODO:



- TODO: TDD

e2e testing - DBAccess - Mocking - Postgres

https://blog.logrocket.com/end-to-end-testing-for-rust-web-services/

https://github.com/JosephLenton/axum-test/blob/b7539c3083a93af0028e9f74e74d59d6980df0dc/examples/example-todo/main.rs

video tutorial
https://www.mindluster.com/lesson/148390

video tutorial full stack
https://www.reddit.com/r/rust/comments/12gx11b/jeremy_chones_rust_axum_full_course/

tdd api rust

https://github.com/drodil/op-api-rust-sdk/blob/main/tests/accounts.rs

tdd api rust

<https://dev.to/rogertorres/rest-api-with-rust-warp-2-post-3527>

unit test rust api

<https://codevoweb.com/how-to-write-unit-tests-for-your-rust-api/>

Tutorial rest rust axum

<https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/>

Docker rust web

<https://codevoweb.com/dockerizing-a-rust-api-project/>


tests web 

<https://github.com/hyperium/hyper/blob/master/examples/client_json.rs>

<https://github.com/hyperium/hyper/blob/master/tests/client.rs>


Rust : Zero to Production

<https://github.com/LukeMathWalker/zero-to-production>

- TODO: upload files
- TODO: execute db index
- TODO: web
- TODO: ssh
- TODO: MQ
- TODO: Dockerfile
- TODO: Kuberentes deployment
- TODO: Functional Programming



----

https://github.com/tokio-rs/axum

https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md

https://rust-on-nails.com/





















