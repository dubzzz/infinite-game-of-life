import { UniverseWasm } from "gol-engine";

enum Mode {
  Move = "move",
  Draw = "draw",
}
type Point = { x: number; y: number };
type Origin = { x: number; y: number; zoom: number };

type MoveAction = {
  mode: Mode.Move;
  initialOrigin: Point;
  initialMousePosition: Point;
};
type DrawAction = {
  mode: Mode.Draw;
};
type Action = MoveAction | DrawAction;

const patterns: Point[][] = [
  [{ x: 0, y: 0 }],
  [
    { x: -1, y: 0 },
    { x: 0, y: 0 },
    { x: 1, y: 0 },
  ],
  [
    { x: -1, y: 0 },
    { x: 0, y: 0 },
    { x: 1, y: 0 },
    { x: 2, y: 0 },
  ],
  [
    { x: -1, y: 0 },
    { x: 0, y: 0 },
    { x: 1, y: 0 },
    { x: -1, y: 1 },
  ],
  [
    { x: -1, y: 0 },
    { x: 0, y: 0 },
    { x: 1, y: 0 },
    { x: -2, y: 1 },
  ],
  [
    { x: -1, y: 0 },
    { x: 0, y: 0 },
    { x: 1, y: 0 },
    { x: 0, y: 1 },
  ],
  [
    { x: -1, y: -1 },
    { x: -1, y: 0 },
    { x: 0, y: -1 },
    { x: 0, y: 1 },
    { x: 1, y: 0 },
  ],
  [
    { x: -1, y: -1 },
    { x: -1, y: 0 },
    { x: 0, y: -1 },
    { x: 0, y: 1 },
    { x: 1, y: 0 },
    { x: 1, y: 1 },
  ],
  [
    { x: -1, y: 0 },
    { x: 0, y: -1 },
    { x: -1, y: 1 },
    { x: 1, y: -1 },
    { x: 0, y: 2 },
    { x: 2, y: 0 },
    { x: 1, y: 1 },
  ],
  [
    { x: 0, y: -1 },
    { x: 1, y: 0 },
    { x: -1, y: 1 },
    { x: 0, y: 1 },
    { x: 1, y: 1 },
  ],
];

class UI {
  readonly #screen: HTMLCanvasElement;
  #universe: UniverseWasm;
  #origin: Origin;
  #halo: Point[];

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
    this.#halo = [];

    // Connect user actions
    let patternIndex = 0;
    let action: Action | undefined = undefined;
    this.#screen.addEventListener("mousedown", (event) => {
      if (event.button !== 0) {
        return;
      }
      this.#halo = [];
      if (event.ctrlKey) {
        const point = this.#pointInScreenToPointInScene({
          x: event.clientX,
          y: event.clientY,
        });
        for (const delta of patterns[patternIndex]) {
          this.#universe = this.#universe.add(
            point.y + delta.y,
            point.x + delta.x
          );
        }
        this.redrawScene();
        action = {
          mode: Mode.Draw,
        };
        return;
      }
      if (event.altKey) {
        const canvasWidth = this.#screen.width;
        const canvasHeight = this.#screen.height;
        const sceneMinX = Math.floor(this.#origin.x / this.#origin.zoom);
        const sceneMaxX = Math.ceil(
          (this.#origin.x + canvasWidth - 1) / this.#origin.zoom
        );
        const sceneMinY = Math.floor(this.#origin.y / this.#origin.zoom);
        const sceneMaxY = Math.ceil(
          (this.#origin.y + canvasHeight - 1) / this.#origin.zoom
        );
        const ys: number[] = [];
        const xs: number[] = [];
        for (let y = sceneMinY; y <= sceneMaxY; ++y) {
          for (let x = sceneMinX; x <= sceneMaxX; ++x) {
            if (Math.random() < 0.25) {
              xs.push(x);
              ys.push(y);
            }
          }
        }
        this.#universe = this.#universe.add_many(
          Int32Array.from(ys),
          Int32Array.from(xs)
        );
        this.redrawScene();
      }
      action = {
        mode: Mode.Move,
        initialOrigin: this.#origin,
        initialMousePosition: { x: event.clientX, y: event.clientY },
      };
    });
    this.#screen.addEventListener("mousemove", (event) => {
      if (action === undefined) {
        this.#halo = [];
        if (event.ctrlKey) {
          const point = this.#pointInScreenToPointInScene({
            x: event.clientX,
            y: event.clientY,
          });
          for (const delta of patterns[patternIndex]) {
            this.#halo.push({ x: point.x + delta.x, y: point.y + delta.y });
          }
          this.redrawScene();
          return;
        }
        return;
      }
      switch (action.mode) {
        case Mode.Move: {
          const dx = event.clientX - action.initialMousePosition.x;
          const dy = event.clientY - action.initialMousePosition.y;
          this.#origin = {
            ...this.#origin,
            x: action.initialOrigin.x - dx,
            y: action.initialOrigin.y - dy,
          };
          this.redrawScene();
          break;
        }
        case Mode.Draw: {
          const point = this.#pointInScreenToPointInScene({
            x: event.clientX,
            y: event.clientY,
          });
          for (const delta of patterns[patternIndex]) {
            this.#universe = this.#universe.add(
              point.y + delta.y,
              point.x + delta.x
            );
          }
          this.redrawScene();
          break;
        }
      }
    });
    this.#screen.addEventListener("mouseup", (event) => {
      if (event.button !== 0) {
        return;
      }
      action = undefined;
    });
    this.#screen.addEventListener("wheel", (event) => {
      event.preventDefault();
      const dzoom = Math.sign(event.deltaY);
      if (
        (event.ctrlKey && action === undefined) ||
        action?.mode === Mode.Draw
      ) {
        patternIndex =
          (patternIndex + dzoom + patterns.length) % patterns.length;

        const point = this.#pointInScreenToPointInScene({
          x: event.clientX,
          y: event.clientY,
        });
        for (const delta of patterns[patternIndex]) {
          this.#halo.push({ x: point.x + delta.x, y: point.y + delta.y });
        }
        this.redrawScene();
        return;
      }
      const newZoom = Math.max(1, this.#origin.zoom - dzoom);
      // We aim for:
      // >  o{x,y}: cell corresponding to the origina of the screen
      // >  c{x,y}: cell corresponding to the position of the mouse
      // >          -> we want it to be the same after the zoom
      // >  cx = ox + event.clientX / zoom
      // >  cx(new) = ox(new) / zoom(new) + clientX(new) / zoom(new) = (ox(new) + clientX) / zoom(new)
      // >  cx(old) = (ox(old) + clientX) / zoom(old)
      // >  -> cx(new) = cx(old)
      // >  -> ox(new) = (zoom(new) / zoom(old)) * (ox(old) + clientX) - clientX
      this.#origin = {
        x:
          (newZoom / this.#origin.zoom) * (this.#origin.x + event.clientX) -
          event.clientX,
        y:
          (newZoom / this.#origin.zoom) * (this.#origin.y + event.clientY) -
          event.clientY,
        zoom: newZoom,
      };
      this.redrawScene();
    });
    document.addEventListener("keyup", (event) => {
      if (!event.ctrlKey && this.#halo.length !== 0) {
        this.#halo = [];
        this.redrawScene();
      }
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

  #pointInScreenToPointInScene(point: Point): Point {
    const cx = Math.floor((this.#origin.x + point.x) / this.#origin.zoom);
    const cy = Math.floor((this.#origin.y + point.y) / this.#origin.zoom);
    return { x: cx, y: cy };
  }

  #pointInSceneToPointInScaledScreen(point: Point): Point {
    const sx = point.x - Math.floor(this.#origin.x / this.#origin.zoom);
    const sy = point.y - Math.floor(this.#origin.y / this.#origin.zoom);
    return { x: sx, y: sy };
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

    const drawPoint = (
      y: number,
      x: number,
      computeNextGreen: (previousGreen: number) => number
    ) => {
      for (let j = 0; j !== this.#origin.zoom; ++j) {
        const cy = y * this.#origin.zoom + j;
        if (cy < 0) {
          continue;
        }
        if (cy >= canvasHeight) {
          break;
        }
        for (let i = 0; i !== this.#origin.zoom; ++i) {
          const cx = x * this.#origin.zoom + i;
          if (cx < 0) {
            continue;
          }
          if (cx >= canvasWidth) {
            break;
          }
          const canvasIndex = (cx + cy * canvasWidth) * 4;
          const data = canvasData.data;
          data[canvasIndex + 0] = 0;
          data[canvasIndex + 1] = computeNextGreen(data[canvasIndex + 1]);
          data[canvasIndex + 2] = 0;
          data[canvasIndex + 3] = 255;
        }
      }
    };

    for (let y = 0; y !== sceneHeight; ++y) {
      for (let x = 0; x !== sceneWidth; ++x) {
        const sceneIndex = x + y * sceneWidth;
        const alive = scene[sceneIndex] === "*";
        drawPoint(y, x, () => (alive ? 255 : 0));
      }
    }
    for (const point of this.#halo) {
      const refined = this.#pointInSceneToPointInScaledScreen(point);
      drawPoint(refined.y, refined.x, (previous) => Math.max(previous, 127));
    }
    ctx.putImageData(canvasData, 0, 0);
  }
}

export function setupUI(element: HTMLDivElement) {
  new UI(element);
}
