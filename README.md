# Shakesemon

## Running Locally

### Docker

Make sure you have [Docker Desktop](https://www.docker.com/products/docker-desktop) installed.  First build the image from root of this repository:

```zsh
docker build --tag shakesemon --file Dockerfile .
```

Then choose a port and run `shakesemon`:

```
export PORT=8000
docker run -e BIND_ADDRESS=0.0.0.0:${PORT} -p ${PORT}:${PORT} shakesemon
```

Now query your favorite pokémon!  E.g. using [curl](https://curl.se):

```
curl localhost:8000/pokemon/pikachu
```

