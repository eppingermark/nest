const worker = new Worker("worker.js", { type: "module" });
import { addTracelog, consoleLog, drawScreen, updateCRom, updatePRom, updateRam, updateVRam } from "./lib.js";

let canvas = document.querySelector("#screen");

worker.onmessage = (e) => {
    let data = e.data;
    switch (data.type) {
        case "forceScreenDraw":
            worker.postMessage({ type: "drawScreen" })
            break;
        case "consoleLog":
            consoleLog(data.msg);
            break;
        case "addTracelog":
            requestAnimationFrame(() => addTracelog(...data.args));
            break;
        case "updateRam":
            requestAnimationFrame(() => updateRam(data.bytes));
            break;
        case "updatePRom":
            requestAnimationFrame(() => updatePRom(data.bytes));
            break;
        case "updateVRam":
            requestAnimationFrame(() => updateVRam(data.bytes));
            break;
        case "updateCRom":
            requestAnimationFrame(() => updateCRom(data.bytes));
            break;
        case "drawScreen":
            requestAnimationFrame(() => drawScreen(data.buffer));
            break;
    }
}

async function run() {
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, 256, 240);

    worker.postMessage({ type: "init", payload: {} });
}

run();
