import { access } from "node:fs/promises";
import { join } from "node:path";

const root = process.cwd();
const requiredFiles = [
  "src/App.tsx",
  "src/main.tsx",
  "src/styles/index.css",
  "src-tauri/Cargo.toml",
  "src-tauri/tauri.conf.json",
  "src-tauri/src/main.rs"
];

await Promise.all(
  requiredFiles.map((file) => access(join(root, file)))
);

console.log("M0 scaffold smoke check passed");
