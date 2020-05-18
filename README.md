# dkr_pub

# Introduction

Github docker container action workflow to build, tag, and push to a remote repository.

## Inputs

### `name`

The name of the docker image to be built.

### `username`

The username of the remote repository (e.g. dockerhub login username)

### `password`

The password of the remote repository (e.g. dockerhub login password)

### `registry`

The url of the remote repository (e.g. github package registry) to push images towards. Default is dockerhub.

## Usage

### Example to push to dockerhub

```
env:
  IMAGE_NAME: "test-image"

jobs:
  tag:
    runs-on: ubuntu-latest

    steps:
      - uses: kennylouie/dkr_pub@v0.0.1
        with:
          name: ${{ env.IMAGE_NAME }}
          username: ${{ secrets.DOCKERHUB_USER }}
          password: ${{ secrets.DOCKERHUB_TOKEN }}
```
