## Plarza Server
### Developing
```bash
cargo install cargo-watch
cargo watch -x server
```
Make sure you are in the root folder of the project when running any Docker commands  
### Running local Docker build
```bash
docker build -t server -f ./server/Dockerfile .
docker run -p 8080:8080 --name server server
```
### Publishing Docker build
```bash
cargo test
docker build -t server -f ./server/Dockerfile .
docker tag server k4yp/server:latest
docker push k4yp/server:latest
```
