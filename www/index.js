import init, {Direction, get_cells, get_height, get_width} from "../pkg/sokoban_rs_wasm.js";

await initialize();

async function initialize() {
    await init();

    let height = get_height();
    let width = get_width();
    let cells = get_cells();
    console.log(cells);

    // TODO add the code to interact with the Wasm logic
    drawCells();

    function unpackCellName(x) {
        let t;
        if (10 <= x && x <= 13) {
            t = `worker-${Direction[x - 10].toLowerCase()}`
        } else if (20 <= x && x <= 23) {
            t = `worker-on-goal-${Direction[x - 20].toLowerCase()}`
        } else if (x === 30) {
            t = "goal"
        } else if (x === 40) {
            t = "box"
        } else if (x === 50) {
            t = "box-on-goal"
        } else if (x === 60) {
            t = "empty"
        } else {
            t = "wall"
        }
        return t;
    }

    function drawCells() {
        // clear game board
        let board = document.getElementById("game-board");
        if (!board) {
            throw new Error("Unable to find #game-board");
        }
        board.style.gridTemplateColumns = `repeat(${width}, 64px)`;
        board.style.gridTemplateRows = `repeat(${height}, 64px)`;

        while (board.hasChildNodes()) {
            board.removeChild(board.lastChild);
        }
        for (let i = 0; i < height; i++) {
            for (let j = 0; j < width; j++) {
                let c = unpackCellName(cells[i * width + j]);
                let d = document.createElement("div");
                d.className = `cell ${c}`;
                board.appendChild(d);
            }
        }
    }
}
