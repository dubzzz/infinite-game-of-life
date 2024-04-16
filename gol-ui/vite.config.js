import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  path: "https://dubzzz.github.io/infinite-game-of-life/",
  plugins: [wasm(), topLevelAwait()],
});
