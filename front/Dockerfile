# pull official base image
FROM node:19-alpine as builder

WORKDIR /front
# ENV PATH /app/node_modules/.bin:$PATH
COPY package.json ./
COPY package-lock.json ./
RUN npm install
# RUN npm install react-scripts@3.4.1 -g
COPY . ./
RUN npm run build

# production environment
FROM nginx:alpine
RUN rm -rf /usr/share/nginx/html/*
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder front/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]