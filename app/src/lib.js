// import WasmMemoryCache from "../../sdk"

import init, { MemoryCache } from '../../pkg';

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

  getAll() {
    return this.cache.get_all();
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

const cache = new WasmMemoryCache()

console.log({ cache })

// const remoteUrl = 'http://remote.sync-engine.local';
// cache.initialise(remoteUrl);
// cache.initialise();
// cache.insert("key", "value")
// console.log({ initialisedCache: cache })
//

function initCache() {
  cache.init()
}

function insertData(key, value) {
  cache.insert(key, value)
}

function getData(key) {
  return cache.get(key)
}

function getAllData() {
  return JSON.parse(cache.getAll())
}

function removeData(key) {
  cache.remove(key)
}

function fetchData(url) {
  cache.fetchFromRemote(url)
}

export {
  getData,
  getAllData,
  fetchData,
  initCache,
  insertData,
  removeData,
}
