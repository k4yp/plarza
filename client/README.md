## Plarza Client
### Developing
```bash
npm install
npm run dev -- --open
```
### Running local Docker build
```bash
docker build -t client .
docker run -p 4173:4173 -d --name client client
```
### Publishing Docker build
```bash
npm run check
docker build -t client .
docker tag client k4yp/client:latest
docker push k4yp/client:latest
```
