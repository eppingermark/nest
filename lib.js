function insertToRow(row, s) {
    const td = document.createElement("td");
    td.innerText = s
    row.appendChild(td)
}

export function fill(id, arr, numCols) {
    const grid = document.getElementById(id);
    const totalRows = Math.ceil(arr.length / numCols);
    let output = "";

    output += "     ";
    for (let i = 0; i < numCols; i++) {
        output += i.toString(16).padStart(2, "0").toUpperCase() + " ";
    }
    output += "\n";

    for (let row = 0; row < totalRows; row++) {
        const rowAddr = (row * numCols).toString(16).padStart(4, "0").toUpperCase();
        output += rowAddr + " ";

        for (let col = 0; col < numCols; col++) {
            const index = row * numCols + col;
            output += index < arr.length
                ? arr[index].toString(16).padStart(2, "0").toUpperCase() + " "
                : "   ";
        }
        output += "\n";
    }

    grid.textContent = output;
}

export function drawScreen(pixels) {
    const canvas = document.querySelector("#screen");
    const ctx = canvas.getContext('2d');

    const imageData = new ImageData(new Uint8ClampedArray(pixels), 256, 240);
    ctx.putImageData(imageData, 0, 0, 0, 0, 256, 240);
}

export function consoleLog(msg) {
    console.log("FROM WASM:", msg);
}

const tracelogAsm = document.querySelector("#tracelog-asm");
const tracelogReg = document.querySelector("#tracelog-reg");
const tracelogs = [];

export function addTracelog(pg, by, inst, regA, regX, regY, sp, p, f, cy) {
    while (tracelogs.length >= 50) {
        tracelogs.shift();
    }

    tracelogs.push({ pg, by, inst, regA, regX, regY, sp, p, f, cy });


    renderTracelog();
}

function renderTracelog() {
    tracelogAsm.innerHTML = "";
    tracelogReg.innerHTML = "";

    tracelogs.forEach(l => {
        const asm = document.createElement("tr");
        insertToRow(asm, l.pg + ":");
        insertToRow(asm, l.by);
        insertToRow(asm, l.inst);
        tracelogAsm.appendChild(asm);

        const reg = document.createElement("tr");
        insertToRow(reg, "A:" + l.regA);
        insertToRow(reg, "X:" + l.regX);
        insertToRow(reg, "Y:" + l.regY)
        insertToRow(reg, "SP:" + l.sp);
        insertToRow(reg, "P:" + l.p);
        insertToRow(reg, l.f);
        insertToRow(reg, "CY:" + l.cy);
        tracelogReg.appendChild(reg);
    })
}

export function updateRam(bytes) {
    fill("ram", bytes, 16)
}

export function updatePRom(bytes) {
    fill("prom", bytes, 16)
}

export function updateVRam(bytes) {
    fill("vram", bytes, 16)
}

export function updateCRom(bytes) {
    fill("crom", bytes, 16)
}
