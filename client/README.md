# Plarza Client
### Developing
```bash
npm run dev
npm run dev -- --open
```
### Publishing Docker build

```bash
docker build -t client .
docker tag client k4yp/client:latest
docker push k4yp/client:latest
```
### Running local Docker build
```bash
docker run -p 4173:4173 -d --name client client
```