import init, { analyze, display, memorize } from '../pkg/bld_tadeo_ca.js';

await init();

/** @type {HTMLInputElement} */
let n = document.querySelector("#n");

/** @type {HTMLTextAreaElement} */
let scrambles = document.querySelector("#scrambles");

/** @type {HTMLTextAreaElement} */
let memo = document.querySelector("#memo");

/** @type {HTMLInputElement} */
let lettering = document.querySelector("#lettering");

/** @type {HTMLOutputElement} */
let analysis = document.querySelector("#analysis");

function update() {
    let N = n.value;
    try {
        analysis.textContent = analyze(
            N, scrambles.value, memo.value, lettering.value,
            document.querySelector("[id=edgebuffer]").value,
            document.querySelector("[id=cornerbuffer]").value,
            document.querySelector("[id=wingbuffer]").value,
            document.querySelector("[id=xcenterbuffer]").value,
            document.querySelector("[id=tcenterbuffer]").value,
            document.querySelector("[id=leftobliquebuffer]").value,
            document.querySelector("[id=rightobliquebuffer]").value
        );
    } catch (e) {
        analysis.textContent = e;
    }

}

n.addEventListener("input", update);
scrambles.addEventListener("input", update);
memo.addEventListener("input", update);
lettering.addEventListener("input", update);
document.querySelector("[id=edgebuffer]").addEventListener("input", update);
document.querySelector("[id=cornerbuffer]").addEventListener("input", update);
document.querySelector("[id=wingbuffer]").addEventListener("input", update);
document.querySelector("[id=xcenterbuffer]").addEventListener("input", update);
document.querySelector("[id=tcenterbuffer]").addEventListener("input", update);
document.querySelector("[id=leftobliquebuffer]").addEventListener("input", update);
document.querySelector("[id=rightobliquebuffer]").addEventListener("input", update);

update();
