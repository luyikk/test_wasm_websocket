import init from './pkg/helloworld.js';

window.addEventListener('load', async () => {
    await init('./pkg/helloworld_bg.wasm');
});