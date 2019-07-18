## Dependencies

- [docker](https://docs.docker.com/install/)
- [docker-compose](https://docs.docker.com/compose/install/)

## Getting started

1. start application by running `docker-compose up --build -d`

The application will be available on `80` port

## FAQ

- **I made a change in frontend directory but when I build its image and start an application, the changes doesn't apply**

  You should delete docker volume by running `docker volume rm mtasa-masterlist_static` every time, when you build new `frontend` docker image
