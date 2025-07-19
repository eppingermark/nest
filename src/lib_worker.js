export function forceScreenDraw(byteArray) {
    postMessage({ type: "forceScreenDraw" });
}

export function consoleLog(msg) {
    postMessage({ type: "consoleLog", msg });
}

export function addTracelog(pg, by, inst, regA, regX, regY, sp, p, f, cy) {
    postMessage({ type: "addTracelog", args: [pg, by, inst, regA, regX, regY, sp, p, f, cy] });
}

export function updateRam(bytes) {
    postMessage({ type: "updateRam", bytes });
}

export function updatePRom(bytes) {
    postMessage({ type: "updatePRom", bytes });
}

export function updateVRam(bytes) {
    postMessage({ type: "updateVRam", bytes });
}

export function updateCRom(bytes) {
    postMessage({ type: "updateCRom", bytes });
}
