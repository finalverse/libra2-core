FROM debian-base AS keyless-pepper-service

COPY --link --from=tools-builder libra2-indexer-grpc-cache-workerlibra2-keyless-pepper-service /usr/local/bin/libra2-keyless-pepper-service

EXPOSE 8000
ENV RUST_LOG_FORMAT=json

# add build info
ARG GIT_TAG
ENV GIT_TAG ${GIT_TAG}
ARG GIT_BRANCH
ENV GIT_BRANCH ${GIT_BRANCH}
ARG GIT_SHA
ENV GIT_SHA ${GIT_SHA}

ENTRYPOINT [ "libra2-keyless-pepper-service" ]
