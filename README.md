# link-book-rust
## Run
```
cargo watch -x run
```

## Build
```
LINK_BOOK_ENV=dev cargo build
LINK_BOOK_ENV=staging cargo build
LINK_BOOK_ENV=production cargo build --release
```

## Docker Build
docker build -f Dockerfile.prod -t link-book .

## Docker Run
docker run -p 4000:8000 link-book
