## Building Docker Image

- docker build . -t rustium-example-image
- docker save rustium-example-image:latest | gzip > rustium-example-image.tar.gz
- docker compose up
