### NFT Metadata Crawler Image ###

FROM indexer-builder

FROM debian-base AS nft-metadata-crawler

COPY --link --from=indexer-builder libra2-indexer-grpc-cache-workerlibra2-nft-metadata-crawler /usr/local/bin/libra2-nft-metadata-crawler

# The health check port
EXPOSE 8080
