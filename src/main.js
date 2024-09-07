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

import * as d3 from "https://cdn.jsdelivr.net/npm/d3@7/+esm";

const width = 400;
const height = 400;
const marginTop = 20;
const marginRight = 20;
const marginBottom = 30;
const marginLeft = 40;


const x = d3.scaleLinear()
    .domain([0, 100])
    .range([marginLeft, width - marginRight]);

const y = d3.scaleLinear()
    .domain([0, 100])
    .range([height - marginBottom, marginTop]);

const line = d3.line()
    .x(d => x(d.date))
    .y(d => y(d.close));

const svg = d3.create("svg")
    .attr("width", width)
    .attr("height", height);

svg.append("g")
    .attr("transform", `translate(0,${height - marginBottom})`)
    .call(d3.axisBottom(x));

svg.append("g")
    .attr("transform", `translate(${marginLeft},0)`)
    .call(d3.axisLeft(y));

svg.append("path")
    .attr("fill", "none")
    .attr("stroke", "steelblue")
    .attr("stroke-width", 1.5)
    .attr("d", line(aapl));

courbe.append(svg.node());
