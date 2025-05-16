const path = require("path");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./src/server/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "server.js",
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "./wgsl-language-server"),
      outDir: path.resolve(__dirname, "./dist/pkg"),
    }),
  ],
  target: "webworker",
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
};
