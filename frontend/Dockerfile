#build frontend

#use the offical node image as base image
FROM node:alpine as build

# Setup the working directory inside the container
WORKDIR /app

#set the env node_modules/.bin
ENV PATH /app/node_modules/.bin:$PATH

#copy the package json file
COPY package.json /app/package.json

# Install all dependencies
RUN npm install --silent

#Copy the files to app folder
COPY . /app

# RUN npm build
RUN npm run build


#move build ot nginx and run the front-end
FROM nginx:alpine

#Copy from build to nginx
COPY --from=build /app/build /usr/share/nginx/html

# Run nginx configuration file
RUN rm /etc/nginx/conf.d/default.conf

# Copy nginx setting to nginx folder
COPY ./nginx/nginx.conf /etc/nginx/conf.d

#Expose thep  port to nginx to run the app
EXPOSE 3001

CMD [ "nginx", "-g", "daemon off;" ]
