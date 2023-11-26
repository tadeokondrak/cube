import init, { display, memorize } from './pkg/bld_tadeo_ca.js';

await init();

/** @type {HTMLInputElement} */
let n = document.querySelector("#n");

/** @type {HTMLInputElement} */
let scramble = document.querySelector("#scramble");

/** @type {HTMLInputElement} */
let lettering = document.querySelector("#lettering");

/** @type {HTMLOutputElement} */
let memo = document.querySelector("#memo");

/** @type {HTMLDivElement} */
let preview = document.querySelector("#preview");

function update() {
    let N = n.value;
    try {
        memo.textContent = memorize(
            N, scramble.value, lettering.value,
            document.querySelector("[id=edgebuffer]").value,
            document.querySelector("[id=cornerbuffer]").value,
            document.querySelector("[id=wingbuffer]").value,
            document.querySelector("[id=xcenterbuffer]").value,
            document.querySelector("[id=tcenterbuffer]").value,
            document.querySelector("[id=leftobliquebuffer]").value,
            document.querySelector("[id=rightobliquebuffer]").value,
        );
    } catch (e) {
        memo.textContent = e;
    }

    try {
        preview.innerHTML = display(N, scramble.value);
    } catch {
        preview.replaceChildren();
    }
}

n.addEventListener("input", update);
scramble.addEventListener("input", update);
lettering.addEventListener("input", update);
document.querySelector("[id=edgebuffer]").addEventListener("input", update);
document.querySelector("[id=cornerbuffer]").addEventListener("input", update);
document.querySelector("[id=wingbuffer]").addEventListener("input", update);
document.querySelector("[id=xcenterbuffer]").addEventListener("input", update);
document.querySelector("[id=tcenterbuffer]").addEventListener("input", update);
document.querySelector("[id=leftobliquebuffer]").addEventListener("input", update);
document.querySelector("[id=rightobliquebuffer]").addEventListener("input", update);

update();
