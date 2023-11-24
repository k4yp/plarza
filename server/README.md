## Plarza Server
### Developing
```bash
pip install -r requirements.txt
flask run --debug
```
### Running local Docker build
```bash
docker run -p 5000:5000 -d --name server server
```
### Publishing Docker build
```bash
docker build -t server .
docker tag server k4yp/server:latest
docker push k4yp/server:latest
```
