## Installation

- docker [[installation]](https://docs.docker.com/get-docker)
- docker-compose [[installation]](https://docs.docker.com/compose/install)

## Define services in a Compose file

- Create a file called docker-compose.yaml in your project directory and paste the following:

```yaml
version: "3.7"

services:
  exchange:
    image: rust-grpc-exchange:1.0.0
    container_name: exchange
    volumes:
      - ./logs/exchange:/app/logs
    ports:
      - "8001:8001"
    restart: unless-stopped
    environment:
      PORT: 8001
```

- This Compose file define exchange service

## Run your app with Compose

```bash
$ cd ops/docker && docker-compose up -d
```

And if you see the following output then services is started ok.
```bash
Creating exchange             ... done
```

## Some other commands

```bash
$ docker-compose ps

       Name                      Command                  State                             Ports                       
------------------------------------------------------------------------------------------------------------------------
exchange              /app/server        Up (healthy)   0.0.0.0:8001->8001/tcp,:::8001->8001/tcp          

$ docker logs exchange

[2021-05-31T12:35:41Z INFO  server] Listening on http://0.0.0.0:8001
```

## Update and start images

```bash
$ docker-compose pull

Pulling exchange   ... done

$ docker-compose up -d 

Recreating exchange ... done
```

## To stop and remove all images and containers

```bash
$ docker-compose down --rmi all

Stopping exchange            ... done
Removing image rust-grpc-exchange:1.0.0
```
