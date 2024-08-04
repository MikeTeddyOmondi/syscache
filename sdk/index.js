import init, { MemoryCache } from '../pkg';

class WasmMemoryCache {
    constructor() {
        this.cache = null;
    }

    async init() {
        await init();
        this.cache = new MemoryCache();
    }

    insert(key, value) {
        this.cache.insert(key, value);
    }

    get(key) {
        return this.cache.get(key);
    }

    remove(key) {
        return this.cache.remove(key);
    }

    async syncWithRemote(url) {
        await this.cache.sync_with_remote(url);
    }

    async fetchFromRemote(url) {
        await this.cache.fetch_from_remote(url);
    }
}

export default WasmMemoryCache;
