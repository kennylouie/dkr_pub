# dkr_pub

# Introduction

Github docker container action workflow to build, tag, and push to a remote repository.

# Usage

## Example to push to dockerhub
uses: actions/dkr_pub@v1
with:
    username: ${{ secrets.DOCKERHUB_USERNAME }}
    username: ${{ secrets.DOCKERHUB_PASSWORD }}
