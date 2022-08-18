FROM node:18-alpine AS builder
WORKDIR /home/node/app
COPY . .

RUN npm install
RUN npm run build


FROM busybox:1.35

RUN adduser -D static
USER static
WORKDIR /home/static

COPY --from=builder /home/node/app/dist .

EXPOSE 80
CMD ["busybox", "httpd", "-f", "-v", "-p", "80"]
