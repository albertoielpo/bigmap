# BigMap
Wrapped class for Rust native HashMap<String,String>. 

With this structure is possible to allocate millions of entry

## Build for source
```bash
# Install cli
yarn global add @napi-rs/cli
# Create a new project
napi new
# Install
yarn install
# Build
yarn build
```

## Test
Create a file main.mjs with the following content and launch it
```js
import { BigMap } from "./index.js";
const bigMap = new BigMap();
bigMap.set("pippo", "pluto");
console.log(bigMap.get("pippo"));   // pluto
console.log(bigMap.len());  // 1
bigMap.clear();
console.log(bigMap.len()); // 0
```