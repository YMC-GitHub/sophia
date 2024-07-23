import { existsSync, mkdirSync } from "fs";
import { dirname, resolve } from "path";

// a library package  to make dirs in js
// recommend-lib-name:touch-util,nanz-make-dirs

/**
 *
 * @sample
 * ```ts
 * makedirs(`docs`)
 *
 * makedirs(`packages/noop`)
 * ```
 */
function makedirs(loc: string) {
  let dir = dirname(resolve(loc));
  if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
}

// read/write file
import { readFileSync, writeFileSync } from "fs";

// // read/write text file
// import { readTextFileSync } from "./nanz-read-text-file-sync";
// import { saveTextFileSync as writeTextFileSync } from "./nanz-save-text-file-sync";
// // read/write text file - json
// import { readJsonFileSync } from "./nanz-read-json-file-sync";
// import { saveJsonFileSync as writeJsonFileSync } from "./nanz-save-json-file-sync";

// read/write text file - yaml
// import yaml from "js-yaml";
// function readYamlFileSymc(loc: string, defaultText: string) {
//   let data: any = yaml.load(readTextFileSync(loc, defaultText));
//   return data;
// }
// function writeYamlFileSymc(loc: string, data: any) {
//   writeTextFileSync(loc, yaml.dump(data));
// }

// // read/write text file - toml
// import toml from "@iarna/toml";
// function readTomlFileSymc(loc: string, defaultText: string) {
//   let data: any = toml.parse(readTextFileSync(loc, defaultText));
//   return data;
// }
// function writeTomlFileSymc(loc: string, data: any) {
//   writeTextFileSync(loc, toml.stringify(data));
// }

export {
  makedirs,
  readFileSync,
  writeFileSync,
  // readTextFileSync,
  // writeTextFileSync,
  // readJsonFileSync,
  // writeJsonFileSync,
  // readYamlFileSymc,
  // writeYamlFileSymc,
  // readTomlFileSymc,
  // writeTomlFileSymc,
};
