import init, { Nes } from "./pkg/nest.js";

let nes;

onmessage = async (e) => {
    const { type, payload } = e.data;

    if (type === "init") {
        await init();
        nes = new Nes();
        await nes.reset();
        clockLoop();
    } else if (type == "drawScreen") {
        const buffer = nes.get_screen_buffer();
        postMessage({ type: "drawScreen", buffer })
    }
};

function clockLoop() {
    if (!nes.is_running()) return;

    const CPU_HZ = 1; // 1789773
    const FPS = 1; // 60
    const CPU_CYCLES_PER_FRAME = Math.floor(CPU_HZ / FPS);
    let cyclesToRun = CPU_CYCLES_PER_FRAME;

    const BATCH_SIZE = 500;

    function runBatch() {
        let batchCycles = 0;
        while (batchCycles < BATCH_SIZE && cyclesToRun > 0) {
            batchCycles += nes.clock();
            cyclesToRun -= batchCycles;
        }

        if (cyclesToRun > 0) {
            setTimeout(runBatch, 0);
        } else {
            requestAnimationFrame(clockLoop);
        }
    }

    runBatch();
}
