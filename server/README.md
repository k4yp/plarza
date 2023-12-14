## Plarza Server
### Developing
```bash
cargo install cargo-watch
cargo watch -x server
```
### Running local Docker build
```bash
docker build -t server .
docker run -p 8080:8080 -d --name server server
```
### Publishing Docker build
```bash
cargo test
docker build -t server .
docker tag server k4yp/server:latest
docker push k4yp/server:latest
```
