FROM ubuntu

RUN apt install g++ -y

WORKDIR /app

COPY . .

RUN make main

CMD /app/main
