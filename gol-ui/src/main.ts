import "./style.css";
import { setupUI } from "./ui.ts";
import init from "gol-engine"; // build it with "wasm-pack build --target web"

init().then(() => setupUI(document.querySelector<HTMLDivElement>("#app")!));
