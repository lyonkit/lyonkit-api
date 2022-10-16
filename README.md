# Lyonkit API

As developer, we have to build a lot of website for people. This project was built because I don't wanted to use Wordpress and I wanted to use frontend frameworks like Vue or Svelte.
Having said that, there is no tools that allow as much customization as Wordpress allows. This is why this project exists.

The core aims of Lyonkit is to allow customization of a website based on props passed to a component.
You just have to build some components for your UI and then allow a website owner is allowed to add component and change it's props.

Additionally, I included a lot of CRUD resources for commonly used features such as _articles_ and _locales_.

The power of lyonkit API is that you can use multiple website for the same endpoint.
Using API keys you can target different website : no need for one backend per project.

This project is for the backend only. For a frontend example see [Lyonkit Vue](https://github.com/leo91000/lyonkit).

## Core concept

- Bloks : This represents a blok on a page, it will be rendered as a component with given props on your website.
- API keys : Api keys are scoped for a single website. One api key can only view resources created using the same api keys. There is also readonly flags for API keys if you need only to read resources (usually your landing page uses a readonly api key while your admin interface will use write api key)

## Requirements

- A 14+ postgresSQL server
- A S3 server to store images

## Deployment

You can deploy the backend using a simple docker command.

```shell
docker run -e DATABASE_URL="..." -e S3__ENDPOINT="..." -e S3__BASE_URL="..." -e S3__CREDENTIALS__ACCESS_KEY_ID="..." -e S3__CREDENTIALS__SECRET_ACCESS_KEY="..." -e  S3__REGION="..." -e CORS="http://myfrontend.com" -p "8080:8080" -d yamakasinge/lyonkit-api
```

## Contributing

### Dev setup

This project is developed in rust. 

#### Requirements

- Docker compose
- rust
- [Just](https://github.com/casey/just) (`cargo install just`)

#### Start

```shell
just start
```

#### Test

```shell
just test
```


