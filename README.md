# dkr_pub

# Introduction

Github docker container action workflow to build, tag, and push to a remote repository.

## Inputs

### `username`

The username of the remote repository (e.g. dockerhub login username)

### `password`

The password of the remote repository (e.g. dockerhub login password)

### `registry`

The url of the remote repository (e.g. github package registry) to push images towards. Default is dockerhub.

## Usage

### Example to push to dockerhub

```
uses: actions/dkr_pub@v1
with:
    username: ${{ secrets.DOCKERHUB_USERNAME }}
    password: ${{ secrets.DOCKERHUB_PASSWORD }}
```
