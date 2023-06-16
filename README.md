# swc-plugin-import-extension-resolver

A SWC plugin to resolve import extensions.

TypeScriptのトランスパイル時にローカルの`.ts`ファイルを`.js`に変換するためのSWCプラグインです。

## Install

```sh
npm install --save-dev swc-plugin-import-extension-resolver
```

or

```sh
yarn add -D swc-plugin-import-extension-resolver
```

or

```sh
pnpm add -D swc-plugin-import-extension-resolver
```

## Usage

### .swcrc

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["swc-plugin-import-extension-resolver", {
          "aliases": [
            "@/*",
            "$/*"
          ]
        }]
      ]
    }
  }
}
```

オプション`aliases`に`jsc.paths`で指定したエイリアスと同じものを指定することで、エイリアスに対しても変換を行なうことができます。
`aliases`は設定せずにオプションなしでも使用可能です。

## Transform example

### Before

```ts
import { NestFactory } from "@nestjs/core";
import { AppModule } from "./app.module";

const bootstrap = async () => {
  const app = await NestFactory.createApplicationContext(AppModule);
  await app.close();
};
bootstrap().catch((error: Error) => {
  throw error;
});
```

### After

```ts
import { NestFactory } from "@nestjs/core";
import { AppModule } from "./app.module.js";
const bootstrap = async ()=>{
    const app = await NestFactory.createApplicationContext(AppModule);
    await app.close();
};
bootstrap().catch((error)=>{
    throw error;
});
```
