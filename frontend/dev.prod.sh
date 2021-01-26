#!/bin/bash
yarn install && yarn build
yarn global add serve
echo "about to serve"
serve -s /srv/app/build -l 3000
