import { parse, newStr, freeBuffer } from "./pkg/js_parsec.js";

const str = new TextEncoder().encode(`["1", 2, 3, 4]`);

const ptr = newStr(str);

const parser = parse(...ptr)

console.log(parser.next());
console.log(parser.next());
console.log(parser.next());
console.log(parser.next());
console.log(parser.next());
console.log(parser.next());

freeBuffer(...ptr);