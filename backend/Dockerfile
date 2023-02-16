FROM elixir:1.14.3-alpine as builder
WORKDIR /source
COPY . .

RUN MIX_ENV=prod mix deps.get
RUN MIX_ENV=prod mix release

RUN mv _build/prod/rel/backend/ /export


FROM alpine:3.17.2
RUN apk add libgcc ncurses libstdc++

WORKDIR /release
COPY --from=builder /export .

ENTRYPOINT ["bin/backend"]
CMD ["start"]
