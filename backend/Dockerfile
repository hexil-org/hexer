FROM elixir:1.14.3-alpine as builder
WORKDIR /source
COPY . .

RUN MIX_ENV=prod mix deps.get
RUN MIX_ENV=prod mix release

RUN mv _build/prod/rel/backend/ /export


FROM elixir:1.14.3-alpine
WORKDIR /release
COPY --from=builder /export .

ENTRYPOINT ["bin/backend"]
CMD ["start"]
