const { invoke } = window.__TAURI__.tauri;

let input;
let result;

async function compute() {
  result.innerHTML = await invoke("compute", { content: input.innerHTML });
}

window.addEventListener("DOMContentLoaded", () => {
  input = document.getElementById("input");
  result = document.getElementById("result");
  input.addEventListener('input', () => {
    compute();
  });
});
