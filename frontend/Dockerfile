# pull official base image
FROM node:lts-alpine

# set work directory
WORKDIR /srv/app/

# add to $PATH
ENV PATH /srv/app/node_modules/.bin:$PATH

# install app dependencies
COPY package.json ./
RUN npm install react-scripts@3.4.1 -g

RUN apk add bash
RUN apk add bash-completion

# add app
COPY . ./
