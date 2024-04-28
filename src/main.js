const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;
const { appWindow} = window.__TAURI__.window;

let unlisten;
let clockEl;

await invoke('init_process')
window.addEventListener("DOMContentLoaded", () => {
  clockEl = document.querySelector("#clock");

})

addEventListener("dblclick", (event) => {
  appWindow.isFullscreen().then(x => appWindow.setFullscreen(!x))
});

unlisten = await listen('clock', (event) => {
  document.querySelector("#clock").textContent = event.payload;
})
