# weekly66
Software Development

---
- Java
- Rust
---

```sh
mvn archetype:generate -DgroupId=com.mycompany.app -DartifactId=my-app -DarchetypeArtifactId=maven-archetype-quickstart -DarchetypeVersion=1.4 -DinteractiveMode=false
```


```sh
mvn compile
mvn test
mvn clean
mvn package

```

<https://start.spring.io/>



```java
// src/main/java/com/example/springboot/HelloController.java
package com.example.springboot;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class HelloController {

	@GetMapping("/")
	public String index() {
		return "Greetings from Spring Boot!";
	}

}
```

```sh
mvn package

┌──(kali㉿kali)-[~/projects/weekly66/devjava/spring-boot]
└─$ java -jar /home/kali/projects/weekly66/devjava/spring-boot/target/spring-boot-0.0.1-SNAPSHOT.jar
```


```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl -v http://localhost:8080/
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
> GET / HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.88.1
> Accept: */*
> 
< HTTP/1.1 200 
< Content-Type: text/plain;charset=UTF-8
< Content-Length: 27
< Date: Fri, 03 Nov 2023 09:43:10 GMT
< 
* Connection #0 to host localhost left intact
Greetings from Spring Boot!   
```



```sh
┌──(kali㉿kali)-[~/projects/weekly66/devjava/spring-boot]
└─$ mvn spring-boot:run
```


- unit test

pom.xml
```xml
<dependency>
	<groupId>org.springframework.boot</groupId>
	<artifactId>spring-boot-starter-test</artifactId>
	<scope>test</scope>
</dependency>
```

<https://spring.io/guides/gs/spring-boot/>


```java
// src/test/java/com/example/springboot/HelloControllerTest.java
package com.example.springboot;

import static org.hamcrest.Matchers.equalTo;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.content;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

import org.junit.jupiter.api.Test;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;

@SpringBootTest
@AutoConfigureMockMvc
public class HelloControllerTest {

	@Autowired
	private MockMvc mvc;

	@Test
	public void getHello() throws Exception {
		mvc.perform(MockMvcRequestBuilders.get("/").accept(MediaType.APPLICATION_JSON))
				.andExpect(status().isOk())
				.andExpect(content().string(equalTo("Greetings from Spring Boot!")));
	}
}
```

```sh
mvn test
```


```java
package com.example.springboot;

import org.junit.jupiter.api.Test;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.test.web.client.TestRestTemplate;
import org.springframework.http.ResponseEntity;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
public class HelloControllerIT {

	@Autowired
	private TestRestTemplate template;

    @Test
    public void getHello() throws Exception {
        ResponseEntity<String> response = template.getForEntity("/", String.class);
        assertThat(response.getBody()).isEqualTo("Greetings from Spring Boot!");
    }
}
```

```xml
        <dependency>
	      <groupId>org.springframework.boot</groupId>
	      <artifactId>spring-boot-starter-actuator</artifactId>
        </dependency>
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66/devjava/spring-boot]
└─$ mvn spring-boot:run
```

```sh
┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl  http://localhost:8080/actuator | jq

┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl  http://localhost:8080/actuator/health | jq

```

```

```

---


- dockerize spring boot java

<https://docs.spring.io/spring-boot/docs/current/reference/htmlsingle/#container-images>


```sh
java -Djarmode=layertools -jar my-app.jar
```

```sh
┌──(kali㉿kali)-[~/…/devjava/spring-boot/target/dependency]
└─$ java -Djarmode=layertools -jar ../*.jar extract
Picked up _JAVA_OPTIONS: -Dawt.useSystemAAFontSettings=on -Dswing.aatext=true
                                                                                                                              
┌──(kali㉿kali)-[~/…/devjava/spring-boot/target/dependency]
└─$ ls
application  dependencies  snapshot-dependencies  spring-boot-loader
```


- Docker in kali linux

<https://www.kali.org/docs/containers/installing-docker-on-kali/#references>
```sh
kali@kali:~$ printf '%s\n' "deb https://download.docker.com/linux/debian bookworm stable" |
  sudo tee /etc/apt/sources.list.d/docker-ce.list

kali@kali:~$ curl -fsSL https://download.docker.com/linux/debian/gpg |
  sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/docker-ce-archive-keyring.gpg

kali@kali:~$ sudo apt update
kali@kali:~$ sudo apt install -y docker-ce docker-ce-cli containerd.io  

```



- Spting Boot Docker

<https://spring.io/guides/topicals/spring-boot-docker/>
```sh

mkdir target/extracted

java -Djarmode=layertools -jar target/*.jar extract --destination target/extracted

docker build -t myorg/myapp .

```


```Dockerfile
FROM eclipse-temurin:17-jdk-alpine
VOLUME /tmp
ARG EXTRACTED=./target/extracted
COPY ${EXTRACTED}/dependencies/ ./
COPY ${EXTRACTED}/spring-boot-loader/ ./
COPY ${EXTRACTED}/snapshot-dependencies/ ./
COPY ${EXTRACTED}/application/ ./
ENTRYPOINT ["java","org.springframework.boot.loader.JarLauncher"]
```


- Index of Java components for fast execution, into the pom.xml

<https://docs.spring.io/spring-framework/reference/core/beans/classpath-scanning.html#beans-scanning-index>

```xml
<dependencies>
	<dependency>
		<groupId>org.springframework</groupId>
		<artifactId>spring-context-indexer</artifactId>
		<version>6.0.13</version>
		<optional>true</optional>
	</dependency>
</dependencies>
```


```sh
java -Djarmode=layertools -jar target/*.jar extract --destination target/extracted

docker build -t myorg/myapp .

┌──(kali㉿kali)-[~/projects/weekly66/devjava/spring-boot]
└─$ docker run -p 8080:8080 myorg/myapp


curl http://localhost:8080/
```




```Dockerfile
FROM eclipse-temurin:17-jdk-alpine as build
WORKDIR .
#RUN addgroup -S kali && adduser -S kali -G kali
#USER kali
COPY mvnw .
COPY .mvn .mvn
COPY pom.xml .
COPY src src

#RUN ./mvnw install -DskipTests
RUN ./mvnw install 
RUN mkdir -p target/dependency && (cd target/dependency; jar -xf ../*.jar)

FROM eclipse-temurin:17-jdk-alpine
VOLUME /tmp
ARG DEPENDENCY=./target/dependency
COPY --from=build ${DEPENDENCY}/BOOT-INF/lib /app/lib
COPY --from=build ${DEPENDENCY}/META-INF /app/META-INF
COPY --from=build ${DEPENDENCY}/BOOT-INF/classes /app


RUN addgroup -S kali && adduser -S kali -G kali
USER kali
ENTRYPOINT ["java","-cp","app:app/lib/*","com.example.demo.UploadingApplication"]
```


```sh
┌──(kali㉿kali)-[~/projects/weekly66/devjava/spring-boot]
└─$ docker build -t myorg/myapp .

┌──(kali㉿kali)-[~/projects/weekly66]
└─$ curl http://localhost:8080/
```



----


```
┌──(kali㉿kali)-[~/projects/weekly66/devjava]
└─$ curl https://start.spring.io/   
```


```
┌──(kali㉿kali)-[~/projects/weekly66/devjava]
└─$ curl -G https://start.spring.io/starter.zip -d type=maven-project -d packaging=jar -d dependencies=web -d name=uploading -o uploading.zip

```



```Dockerfile
FROM eclipse-temurin:17-jdk-alpine as build
WORKDIR .
#RUN addgroup -S kali && adduser -S kali -G kali
#USER kali
COPY mvnw .
COPY .mvn .mvn
COPY pom.xml .
COPY src src

#RUN ./mvnw install -DskipTests
RUN ./mvnw install 
RUN mkdir -p target/dependency && (cd target/dependency; jar -xf ../*.jar)

FROM eclipse-temurin:17-jdk-alpine
VOLUME /tmp
ARG DEPENDENCY=./target/dependency
COPY --from=build ${DEPENDENCY}/BOOT-INF/lib /app/lib
COPY --from=build ${DEPENDENCY}/META-INF /app/META-INF
COPY --from=build ${DEPENDENCY}/BOOT-INF/classes /app


RUN addgroup -S kali && adduser -S kali -G kali
USER kali
ENTRYPOINT ["java","-cp","app:app/lib/*","com.example.demo.UploadingApplication"]
```

TODO: Create the web project, REST, with Dockerfile for deployment.. ( into kubernetes over gitops )


--------
--------
--------

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
TODO:

tests web 

<https://github.com/hyperium/hyper/blob/master/tests/client.rs>


Rust : Zero to Production

<https://github.com/LukeMathWalker/zero-to-production>



TODO: TDD
TODO: upload files
TODO: execute db index
TODO: web
TODO: ssh
TODO: MQ
TODO: Dockerfile
TODO: Kuberentes deployment
TODO: Functional Programming



----

https://github.com/tokio-rs/axum

https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md

https://rust-on-nails.com/





















