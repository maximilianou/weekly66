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
#ENTRYPOINT ["java","org.springframework.boot.loader.JarLauncher"]
