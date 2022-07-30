const ffi = require("ffi-napi");
const ref = require("ref-napi");
const array = require("ref-array-napi");
const StringArray = array("string", 5);

let dir = process.argv[2];

const lib = new ffi.Library("../coursify_util.dll", {
  get_sections: [StringArray, []],
  resolve_sections: ["string", ["int"]],
});

const data = lib.resolve_sections(10);
console.log(data);
