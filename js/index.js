import("../pkg/index.js").then((wasm) => {
    wasm.greet();
}).catch(console.error);
