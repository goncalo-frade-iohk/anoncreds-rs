const {
    LinkSecret
} = require('./wasm/pkg');


const ls = new LinkSecret();
const ls2 = LinkSecret.fromString(ls.toString())
debugger;