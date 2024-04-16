import { UniverseWasm } from "gol-engine";

enum Mode {
  Move = "move",
}
type Point = { x: number; y: number };
type Origin = { x: number; y: number; zoom: number };

type MoveAction = {
  mode: Mode.Move;
  initialOrigin: Point;
  initialMousePosition: Point;
};

class UI {
  readonly #screen: HTMLCanvasElement;
  #universe: UniverseWasm;
  #origin: Origin;

  constructor(element: HTMLDivElement) {
    // Setup empty screen
    this.#screen = document.createElement("canvas");
    this.#screen.style.position = "fixed";
    this.#screen.style.left = "0";
    this.#screen.style.right = "0";
    this.#screen.style.height = "100vh";
    this.#screen.style.width = "100vw";
    this.#screen.className = `mode-${Mode.Move}`;
    element.appendChild(this.#screen);
    this.#syncScreenSize();

    // Setup empty Universe
    this.#universe = UniverseWasm.new();
    this.#universe = this.#universe.add(10, 10);
    this.#universe = this.#universe.add(11, 10);
    this.#universe = this.#universe.add(12, 10);

    // Place origin
    this.#origin = { x: 0, y: 0, zoom: 20 };

    // Connect user actions
    let action: MoveAction | undefined = undefined;
    this.#screen.addEventListener("mousedown", (event) => {
      if (event.button !== 0) {
        return;
      }
      action = {
        mode: Mode.Move,
        initialOrigin: this.#origin,
        initialMousePosition: { x: event.clientX, y: event.clientY },
      };
    });
    this.#screen.addEventListener("mousemove", (event) => {
      if (action === undefined || action.mode !== Mode.Move) {
        return;
      }
      const dx = event.clientX - action.initialMousePosition.x;
      const dy = event.clientY - action.initialMousePosition.y;
      this.#origin = {
        ...this.#origin,
        x: action.initialOrigin.x - dx,
        y: action.initialOrigin.y - dy,
      };
      this.redrawScene();
    });
    this.#screen.addEventListener("mouseup", (event) => {
      if (event.button !== 0) {
        return;
      }
      action = undefined;
    });
    this.#screen.addEventListener("wheel", (event) => {
      console.log(event.deltaY);
      const dzoom = Math.sign(event.deltaY);
      this.#origin = {
        ...this.#origin,
        zoom: Math.max(1, this.#origin.zoom + dzoom),
      };
      this.redrawScene();
    });

    // Connect drawing and observers
    const screenObserver = new ResizeObserver(() => {
      this.#syncScreenSize();
      this.redrawScene();
    });
    screenObserver.observe(this.#screen);
    this.#drawFirstScene();
  }

  #syncScreenSize() {
    const dimensions = this.#screen.getBoundingClientRect();
    this.#screen.height = dimensions.height;
    this.#screen.width = dimensions.width;
  }

  #drawFirstScene() {
    this.redrawScene();
    setTimeout(() => {
      this.#universe = this.#universe.next();
      this.#drawFirstScene();
    }, 250);
  }

  redrawScene() {
    const canvasWidth = this.#screen.width;
    const canvasHeight = this.#screen.height;
    const sceneMinX = Math.floor(this.#origin.x / this.#origin.zoom);
    const sceneMaxX = Math.ceil(
      (this.#origin.x + canvasWidth - 1) / this.#origin.zoom
    );
    const sceneWidth = sceneMaxX - sceneMinX + 1;
    const sceneMinY = Math.floor(this.#origin.y / this.#origin.zoom);
    const sceneMaxY = Math.ceil(
      (this.#origin.y + canvasHeight - 1) / this.#origin.zoom
    );
    const sceneHeight = sceneMaxY - sceneMinY + 1;
    const scene = this.#universe.window(
      sceneMinY,
      sceneMinX,
      sceneMinY + sceneHeight - 1,
      sceneMinX + sceneWidth - 1
    );
    const ctx = this.#screen.getContext("2d")!;
    const canvasData = ctx.getImageData(0, 0, canvasWidth, canvasHeight);
    for (let y = 0; y !== sceneHeight; ++y) {
      for (let x = 0; x !== sceneWidth; ++x) {
        const sceneIndex = x + y * sceneWidth;
        const alive = scene[sceneIndex] === "*";

        for (let j = 0; j !== this.#origin.zoom; ++j) {
          const cy = y * this.#origin.zoom + j;
          if (cy >= canvasHeight) {
            break;
          }
          for (let i = 0; i !== this.#origin.zoom; ++i) {
            const cx = x * this.#origin.zoom + i;
            if (cx >= canvasWidth) {
              break;
            }
            const canvasIndex = (cx + cy * canvasWidth) * 4;
            canvasData.data[canvasIndex + 0] = 0;
            canvasData.data[canvasIndex + 1] = alive ? 255 : 0;
            canvasData.data[canvasIndex + 2] = 0;
            canvasData.data[canvasIndex + 3] = 255;
          }
        }
      }
    }
    ctx.putImageData(canvasData, 0, 0);
  }
}

export function setupUI(element: HTMLDivElement) {
  new UI(element);
}
