### NFT Metadata Crawler Image ###

FROM indexer-builder

FROM debian-base AS nft-metadata-crawler

COPY --link --from=indexer-builder libra2-indexer-grpc-cache-workeraptos-nft-metadata-crawler /usr/local/bin/aptos-nft-metadata-crawler

# The health check port
EXPOSE 8080
