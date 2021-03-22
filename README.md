
![pipeline](https://github.com/olidacombe/shakesemon/actions/workflows/general.yml/badge.svg)
# Shakesemon

Provides a Shakespearean description of named pokémon species.

Leverages the following upstream APIs:
- [funtranslations Shakespeare translator](https://funtranslations.com/api/shakespeare)
- [PokéAPI](https://pokeapi.co/)

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

### Cargo

Alternatively, if you have a [Rust](https://www.rust-lang.org/) toolchain installed, you can compile and run the service using:

```
cargo run
```

## Example Usage

To query your favorite pokémon using [curl](https://curl.se) when running locally as described above:

```
$ curl -s localhost:8000/pokemon/pikachu
{
  "name": "pikachu",
  "description": "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms."
}
```

## Configuration

You can configure the service with the following environment variables:

+ `BIND_ADDRESS` - ip/port specification for binding, defaults to `127.0.0.1:0` (i.e. loopback address with port assigned by the OS)
+ `SHAKESPEARE_TRANSLATOR_URI` - specify alternative translator endpoint (e.g. if you want to point at a caching proxy that might also tack on an api key for you)

## Improvements

Items I'd like to take care of in the future include:

+ Build a release image in-pipeline and run some integration tests against it using real endpoints
  - Conditionally push to [dockerhub](https://hub.docker.com/) on specific tags / branches.
+ Take a more exhaustive description-retrieval approach.
  - Currently I take the first english description found.
  - There are duplicate english descriptions for plenty of species'.
  - __Solution__: filter, de-duplicate and concatenate the english descriptions.
+ Better Error Handling - be more informative to the api consumers.
  - The tests for handling various upstream error states should be much more complete
  - Translate upstream errors better (i.e. inspect the [funtranslations](https://funtranslations.com/shakespeare) more than simply trying to de-serialize `response.contents.translated` and returning "something failed in translation").
  - Currently only rate-limit errors from [funtranslations](https://funtranslations.com/shakespeare) propagate to meaningful error responses
+ Better Logging - some descriptive console logging at the very least.
+ Include more environment variable configuration overrides for:
    - Api keys for use with upstream endpoints (e.g. `X-Funtranslations-Api-Secret`)
    - PokéApi alternative endpoint
+ Re-use any `X-Funtranslations-Api-Secret` header in the client request for users who are paid-up funtranslationistas.
+ Abandon use of [pokerust](https://gitlab.com/olidacombe/pokerust) as it provides little convenience and I had to fork it to gracefully handle a shape discrepancy with a new `is_legendary` property on `PokemonSpecies`.