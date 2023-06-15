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
        ["swc-plugin-import-extension-resolver", {}]
      ]
    }
  }
}
```
