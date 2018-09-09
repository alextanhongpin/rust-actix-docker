# rust-actix

## Build

```bash
$ make build
build:
	docker build -t alextanhongpin/rust-actix .
```

## Run

```bash
$ make run
run:
	docker run -p 8000:8000 alextanhongpin/rust-actix
```

Output:

```
Started http server: 0.0.0.0:8000
```

## Test

```bash
$ curl -XPOST -H 'Content-Type: application/json' -d '{"name": "john", "number": 1}' http://0.0.0.0:8000/hello
{"name":"john","number":1}%
```

## Docker Size

```bash
$ docker images | grep actix
```
Output:
```
alextanhongpin/rust-actix          latest                  c5aa959940a9        8 minutes ago       16.5MB
```
