// // @ts-check

let n = /** @type {HTMLInputElement} */ (document.querySelector("#n"))
let lettering = /** @type {HTMLInputElement} */ (document.querySelector("#lettering"))
let piecetype = /** @type {HTMLInputElement} */ (document.querySelector("#piecetype"))
let search = /** @type {HTMLInputElement} */ (document.querySelector("#search"))
let output = /** @type {HTMLDivElement} */ (document.querySelector("#output"))

const cache = {}

const edgeStickers = [
    "ub",
    "ur",
    "uf",
    "ul",
    "lu",
    "lf",
    "ld",
    "lb",
    "fu",
    "fr",
    "fd",
    "fl",
    "ru",
    "rb",
    "rd",
    "rf",
    "bu",
    "bl",
    "bd",
    "br",
    "df",
    "dr",
    "db",
    "dl",
]

const cornerStickers = [
    "ubl",
    "ubr",
    "ufr",
    "ufl",
    "lub",
    "luf",
    "ldf",
    "ldb",
    "ful",
    "fur",
    "fdr",
    "fdl",
    "ruf",
    "rub",
    "rdb",
    "rdf",
    "bur",
    "bul",
    "bdl",
    "bdr",
    "dfl",
    "dfr",
    "dbr",
    "dbl",
]

function pieceNotationTriple(speffz, f) { return speffz.split("").map(f).join(" ") }
function cornerPieceNotation(speffz) { return cornerStickers[speffz.charCodeAt(0) - "A".charCodeAt(0)].toUpperCase() }
function edgePieceNotation(speffz) { return edgeStickers[speffz.charCodeAt(0) - "A".charCodeAt(0)].toUpperCase() }
function wingPieceNotation(speffz) { return edgeStickers[speffz.charCodeAt(0) - "A".charCodeAt(0)].toUpperCase() }

async function getDataForPieceType(type) {
    if (type in cache) return cache[type];
    const res = await (await fetch(`${type}.json?v4`)).json()
    cache[type] = res;
    return res;
}

/**
 * @param {string} lettering
 * @param {string} triple
 * @returns {string}
 */
function toCustom(lettering, triple) {
    return triple.split("").map(c => lettering[c.charCodeAt(0) - "A".charCodeAt(0)]).join("")
}

/**
 * @param {string} lettering
 * @param {string} triple
 * @returns {string}
 */
function toSpeffz(lettering, triple) {
    return triple.split("").map(c => String.fromCharCode("A".charCodeAt(0) + lettering.indexOf(c))).join("")
}

async function update() {
    try {
        const origSearchValue = search.value.toUpperCase()
        if (!/^[A-Z]{3}$/.test(origSearchValue)) {
            m.render(output, "invalid search")
            return;
        }
        const searchValue = canonicalSearch(toSpeffz(lettering.value, origSearchValue))
        const data = await getDataForPieceType(piecetype.value)
        if (!(searchValue in data) && !(invertSearchValue(searchValue) in data)) {
            m.render(output, "not found")
            return;
        }

        m.render(output, [
            m("div", { style: { display: "flex", flexDirection: "column", gap: "1em" } }, [
                searchValue in data ? m("div", { style: { display: "flex", flexDirection: "column" } }, [
                    m("h3", `${origSearchValue}`),
                    table(data, searchValue),
                ]) : undefined,
                invertSearchValue(searchValue) in data ? m("div", { style: { display: "flex", flexDirection: "column" } }, [
                    m("h3", `${invertSearchValue(origSearchValue)} (inverse)`),
                    table(data, invertSearchValue(searchValue))
                ]) : undefined,
            ])
        ])
    } catch (e) {
        m.render(output, String(e))
        throw e;

    }

    //output.textContent = JSON.stringify(data[searchValue], null, 2);

}


function cell(x) {
    let s = "";
    while (x > 0) {
        s = String.fromCharCode("A".charCodeAt("0") + (x % 26)) + s;
        x /= 26;
        x = Math.floor(x)
    }
    return s;
}

function table(data, searchValue) {
    const algs = Object.entries(data[searchValue]);
    console.log(algs)
    algs.sort(([a_alg, a], [b_alg, b]) => {
        const a_count = Object.entries(a.variants).reduce((acc, [_, { users }]) => acc + Object.entries(users).length, 0);
        const b_count = Object.entries(b.variants).reduce((acc, [_, { users }]) => acc + Object.entries(users).length, 0);
        if (a_count != b_count)
            return b_count - a_count;
        if (a_alg < b_alg) return -1;
        if (a_alg > b_alg) return 1;
        return 0;
    })

    return m("table", { className: "table table-bordered", style: { borderCollapse: "collapse" } }, [
        m("tbody", algs.map(([alg, { variants }]) => {
            const variantList = Object.entries(variants)
            variantList.sort(([_a, a], [_b, b]) => Object.entries(b.users).length - Object.entries(a.users).length)
            const totalUsers = new Set(Object.values(variants).flatMap(({ users }) => Object.keys(users))).size;
            return m("tr", [
                m("td", { className: "d-none d-sm-table-cell p-3 text-secondary", style: { width: "15%" } }, alg),
                m("td", { className: "p-0 m-0", style: { borderBottom: 0 } },
                    m("div", { className: "comms-inner-table", style: { display: "flex", flexDirection: "column" } }, variantList.map(([variant, { notes, users }]) => {
                        const userCount = Object.entries(users).length
                        return m("div", { className: "p-3", style: { display: "flex", flexDirection: "row", justifyContent: "space-between", alignItems: "center", gap: "0" } }, [
                            m("div", { className: "font-monospace", style: {} }, variant),
                            m("div", { className: "d-block d-md-none" }, userCount),
                            m("div", { className: "d-none d-md-block", style: {} },
                                [
                                    m("ul", { className: "text-end", style: { listStyleType: "none", padding: 0, margin: 0, } }, Object.entries(users).map(([user, sources]) => {
                                        return [
                                            m("li", [
                                                `${user} [`,
                                                ...sources.flatMap((source, i) => [
                                                    ...(i > 0 ? [`, `] : []),
                                                    ...("google_sheets" in source ?
                                                        [
                                                            m(
                                                                "a",
                                                                {
                                                                    href: `https://docs.google.com/spreadsheets/d/${source.google_sheets.workbook_id}/edit`,
                                                                    className: "link-underline link-underline-opacity-0 link-underline-opacity-75-hover",
                                                                    title: `${source.google_sheets.sheet_name}, ${cell(source.google_sheets.y)}${source.google_sheets.x + 1}`
                                                                },
                                                                `${i + 1}`
                                                                //
                                                            ),
                                                            //`, `
                                                        ] : "custom" in source ? [
                                                            m(
                                                                "a",
                                                                { href: source.custom.url, className: "link-underline link-underline-opacity-0 link-underline-opacity-75-hover", title: source.custom.name },
                                                                `${i + 1}`
                                                            ),
                                                        ] : ["(error)"]),
                                                ]),
                                                `]`
                                            ])
                                        ]
                                    })
                                    )
                                ],
                            )
                        ])
                    }))
                ),
            ])
        })),
    ]);

}

function canonicalSearch(triple) {
    let count = 0;
    while (count++ < 10 && triple[1] < triple[0] || triple[2] < triple[0]) {
        triple = [triple[1], triple[2], triple[0]].join("");
        count++;
    }
    return triple;

}

function invertSearchValue(triple) {
    return [triple[0], triple[2], triple[1]].join("");
}

search.addEventListener("input", update)
lettering.addEventListener("input", update)
piecetype.addEventListener("input", update)

function pluralMarker(n) {
    if (n == 1) return "";
    return "s";
}

lettering.addEventListener("input", function () {
    if (/^[A-Z]{24}$/.test(lettering.value)) {
        localStorage.lettering = lettering.value
    }
})

if (/^[A-Z]{24}$/.test(localStorage.lettering)) {
    lettering.value = localStorage.lettering
}

update();
