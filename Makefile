build:
	docker build -t alextanhongpin/rust-actix .

run:
	docker run -p 8000:8000 alextanhongpin/rust-actix

