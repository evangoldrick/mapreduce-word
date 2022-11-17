FROM ubuntu

RUN mkdir /app/

WORKDIR /app/

EXPOSE 8080

COPY Rocket.toml /app/

COPY target/debug/first /app/

CMD ./first
