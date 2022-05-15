# wasm-reed-solomon-erasure

This is a WebAssembly compiled version of the [reed-solomon-erasure](https://github.com/rust-rse/reed-solomon-erasure)
library made by Rust to use Reed-Solomon erasure coding at a minimum in TypeScript/JavaScript.

### How To Install

```
npm i wasm-reed-solomon-erasure
```

### Usage

Example of use. This code converts the string "Hello World" into several shards.
(By default, it splits the string into 8 data shards and 2 parity shards.)
We then verify that the two shards can be recovered even if they are destroyed.

```ts
import {encode, reconstruct} from 'wasm-reed-solomon-erasure';

export const bufferToShards = (buf: Buffer, shardsCount = 10, parityShards = 2): Uint8Array[] => {
  const dataShards = shardsCount - parityShards;

  const input = new Uint8Array(buf);

  const shardSize = Math.ceil(input.length / dataShards);
  const shardData = [];

  for (let i = 0; i < dataShards; i++) {
    const array = new Uint8Array(shardSize);
    shardData.push(array);
  }

  for (let i = 0; i < input.length; i++) {
    const j = Math.floor(i / shardSize);
    const k = i % shardSize;
    shardData[j][k] = input[i];
  }


  return encode(shardData, parityShards);
};


export const shardsToBuffer = (shards: Uint8Array[], parityShards: number, deadSharedIndexes: number[]): Buffer => {

  const result = reconstruct(
    shards,
    parityShards,
    new Uint32Array(deadSharedIndexes),
  );

  const flatten = [];
  const dataShards = shards.length - parityShards;
  for (let i = 0; i < dataShards; i++) {
    for (const v of result[i]) flatten.push(v);
  }
  return Buffer.from(flatten);
};
```

and

```ts
import {bufferToShards, shardsToArrayBuffer} from './reedSolomon';
const buf = Buffer.from('Hello World', 'utf-8');
const shards = bufferToShards(buf);
const corrupted = [...shards];
// Shard Destruction
corrupted[1] = new Uint8Array();
corrupted[2] = new Uint8Array();
const restoredBuffer = shardsToArrayBuffer(corrupted, 2, [1, 2]);
const array = new Uint8Array(restoredBuffer);
const restored = Buffer.from(array).toString('utf-8', 0, 'Hello World'.length);
console.log(restored); // Hello World
```


### Notice

I have not done any error handling at all and will implement it next time.

# Developing

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --target nodejs
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --chrome
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish 
```

# ref:
- [npm](https://www.npmjs.com/package/wasm-reed-solomon-erasure)