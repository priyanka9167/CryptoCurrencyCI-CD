# Use offical nginx base image
FROM nginx:alpine

#Delete the default setting 
RUN rm /etc/nginx/conf.d/default.conf

# Copy the nginx conf 
COPY ./nginx.conf /etc/nginx/conf.d