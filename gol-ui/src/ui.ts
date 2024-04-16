import { UniverseWasm } from "gol-engine";

type Point = { x: number; y: number };

class UI {
  readonly #screen: HTMLCanvasElement;
  #universe: UniverseWasm;
  #origin: Point;

  constructor(element: HTMLDivElement) {
    // Setup empty screen
    this.#screen = document.createElement("canvas");
    this.#screen.style.position = "fixed";
    this.#screen.style.left = "0";
    this.#screen.style.right = "0";
    this.#screen.style.height = "100vh";
    this.#screen.style.width = "100vw";
    element.appendChild(this.#screen);

    // Setup dummy Universe
    this.#universe = UniverseWasm.new();
    this.#universe = this.#universe.add(10, 10);
    this.#universe = this.#universe.add(11, 10);
    this.#universe = this.#universe.add(12, 10);
    this.#universe = this.#universe.add(10, 12);
    this.#universe = this.#universe.add(11, 12);
    this.#universe = this.#universe.add(12, 12);
    this.#universe = this.#universe.add(10, 14);
    this.#universe = this.#universe.add(11, 14);
    this.#universe = this.#universe.add(12, 14);

    // Place origin
    this.#origin = { x: 0, y: 0 };

    // Connect drawing
    this.#drawFirstScene();
  }

  #drawFirstScene() {
    this.redrawScene();
    setTimeout(() => {
      this.#universe = this.#universe.next();
      this.#drawFirstScene();
    }, 25);
  }

  redrawScene() {
    const canvasWidth = this.#screen.width;
    const canvasHeight = this.#screen.height;
    const scene = this.#universe.window(
      this.#origin.y,
      this.#origin.x,
      this.#origin.y + canvasHeight,
      this.#origin.x + canvasWidth
    );
    const ctx = this.#screen.getContext("2d")!;
    const canvasData = ctx.getImageData(0, 0, canvasWidth, canvasHeight);
    for (let j = 0; j !== canvasHeight; ++j) {
      for (let i = 0; i !== canvasWidth; ++i) {
        const sceneIndex = i + j * canvasWidth;
        const alive = scene[sceneIndex] === "*";

        const canvasIndex = sceneIndex * 4;
        canvasData.data[canvasIndex + 0] = 0;
        canvasData.data[canvasIndex + 1] = alive ? 255 : 0;
        canvasData.data[canvasIndex + 2] = 0;
        canvasData.data[canvasIndex + 3] = 255;
      }
    }
    ctx.putImageData(canvasData, 0, 0);
  }
}

export function setupUI(element: HTMLDivElement) {
  new UI(element);
}
