# weekly66
Software Development


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

```
src/main.rs
src/lib.rs
src/bin
```



https://github.com/tokio-rs/axum

https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md

https://rust-on-nails.com/





















