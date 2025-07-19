use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/lib_worker.js")]
unsafe extern "C" {
    pub fn consoleLog(msg: &str);
    pub fn addTracelog(pg: &str, by: &str, inst: &str, regA: &str, regX: &str, regY: &str, sp: &str, p: &str, f: &str, cy: &str);
    pub fn updateRam(ram: &[u8]);
    pub fn updatePRom(ram: &[u8]);
    pub fn updateVRam(ram: &[u8]);
    pub fn updateCRom(ram: &[u8]);
    pub fn forceScreenDraw();
}
