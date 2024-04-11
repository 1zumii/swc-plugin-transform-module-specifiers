# swc-plugin: transform-module-specifiers

transform module specifier in `import`, `export` statement, such as `ts` ➡️ `js`

```ts
// Before
import { v1, f2 } from "./file3.ts";
export { v4, f5 } from "../file6.ts";
```
would be transformed to
```js
// after
import { v1, f2 } from "./file3.js";
export { v4, f5 } from "../file6.js";
```

## 🛠️ Config
```shell
npm install -D swc-plugin-transform-module-specifiers
```

```json5
// .swcrc
{
    "plugins": [
        [
            "swc-plugin-transform-module-specifiers",
            {}
        ]
    ]
}
```

by default, only transform `ts` to `js`. declare extension map in plugin's config to custom transform behavior
```json
{
    "mts": "mjs",
    "tsx": "jsx"
}
```

## 🤔 Why this plugin? or what problem it solved

if you are developing in a Node.js + TypeScript + ESM project, and transpile codes by tsc. you might come across: 

```ts
import { something } from "./some-file.ts";
```

tsc errors that:

```
An import path can only end with a '.ts' extension when 'allowImportingTsExtensions' is enabled.ts(5097)
```

but it's wired to import `"./some-file.js"` in the source TS file, as the file doesn’t exist during dev time

solution for this is to change some options in `tsconfig.json`

```json
{
    "compilerOptions": {
        "strict": true,
        "module": "Node16",
        "moduleResolution": "Node16",
        "noEmit": true,
        "allowImportingTsExtensions": true,
    }
}
```

the problem is, now you cannot produce any js code. Then turn to SWC to get these transpile work done, however SWC leaves import/export file extensions **untouched**.

so here comes this plugin.

### References
- [TypeScript imitates the host’s module resolution, but with types](https://www.typescriptlang.org/docs/handbook/modules/theory.html#typescript-imitates-the-hosts-module-resolution-but-with-types)
- [TypeScript 5.0: new mode bundler & ESM](https://dev.to/ayc0/typescript-50-new-mode-bundler-esm-1jic)
- [The TSConfig Cheat Sheet](https://www.totaltypescript.com/tsconfig-cheat-sheet)

> [!NOTE]
> 
> you might not need this plugin, if your project are using:
> - bundler
>   
>   set `tsconfig.json` (refer to above cheat sheet)
>   - `"module": "Preserve"`
>   - `"moduleResolution": "Bundler"`
> 
> - runtime 
> 
>   that able to consume TS file directly