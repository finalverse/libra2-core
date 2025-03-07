### Node Checker Image ###

FROM debian-base AS node-checker

COPY  --link --from=tools-builder libra2-indexer-grpc-cache-workerlibra2-node-checker /usr/local/bin/libra2-node-checker

ENV RUST_LOG_FORMAT=json

# add build info
ARG BUILD_DATE
ENV BUILD_DATE ${BUILD_DATE}
ARG GIT_TAG
ENV GIT_TAG ${GIT_TAG}
ARG GIT_BRANCH
ENV GIT_BRANCH ${GIT_BRANCH}
ARG GIT_SHA
ENV GIT_SHA ${GIT_SHA}
