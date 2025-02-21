#!/usr/bin/env -S pnpm test release-images.test.js

import { assertTagMatchesSourceVersion, getImageReleaseGroupByImageTagPrefix, isReleaseImage } from '../release-images.mjs';

describe('releaseImages', () => {
    it('gets libra2-node as the default image group', () => {
        const prefix = 'image-banana';
        const releaseGroup = getImageReleaseGroupByImageTagPrefix(prefix);
        expect(releaseGroup).toEqual('libra2-node');
    });
    it('gets indexer image group', () => {
        const prefix = 'aptos-indexer-grpc-vX.Y.Z';
        const releaseGroup = getImageReleaseGroupByImageTagPrefix(prefix);
        expect(releaseGroup).toEqual('aptos-indexer-grpc');
    });
    it('gets libra2-node as the node image group', () => {
        const prefix = 'libra2-node-vX.Y.Z';
        const releaseGroup = getImageReleaseGroupByImageTagPrefix(prefix);
        expect(releaseGroup).toEqual('libra2-node');
    });
    it('determines image is a release image', () => {
        expect(isReleaseImage("nightly-banana")).toEqual(false);
        expect(isReleaseImage("libra2-node-v1.2.3")).toEqual(true);
    });
    it('asserts version match', () => {
        // toThrow apparently matches a prefix, so this works but it does actually test against the real config version
        // Which... hilariously means this would fail if the version was ever 0.0.0
        expect(() => assertTagMatchesSourceVersion("libra2-node-v0.0.0")).toThrow("image tag does not match cargo version: libra2-node-v0.0.0");
    });
});
