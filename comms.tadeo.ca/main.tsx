import * as m from "mithril";
//
//let piecetype: HTMLInputElement = document.querySelector("#piecetype")!;
//let search: HTMLInputElement = document.querySelector("#search")!;
let app: HTMLDivElement = document.querySelector("#app")!;

type Data = {
  cases: Record<string, Record<string, Record<string, number[]>>>;
  sources: {
    spreadsheet: {
      user: string;
    };
  }[];
};

const cache: Record<string, Data> = {};

async function getDataForPieceType(type: string) {
  if (type in cache) return cache[type];
  const res = await (await fetch(`data/${type}.json`)).json();
  cache[type] = res;
  return res;
}

// prettier-ignore
const edgeStickers = ["ub", "ur", "uf", "ul", "lu", "lf", "ld", "lb", "fu", "fr", "fd", "fl", "ru", "rb", "rd", "rf", "bu", "bl", "bd", "br", "df", "dr", "db", "dl"];

// prettier-ignore
const cornerStickers = ["ubl", "ubr", "ufr", "ufl", "lub", "luf", "ldf", "ldb", "ful", "fur", "fdr", "fdl", "ruf", "rub", "rdb", "rdf", "bur", "bul", "bdl", "bdr", "dfl", "dfr", "dbr", "dbl"];

let searchInputValue = "";
let piecetypeValue = "Corner3Cycle";

function doIt(): Promise<JSX.Element> {
  const origSearchValue = searchInputValue.toUpperCase();
  if (!/^[A-Z]{3}$/.test(origSearchValue)) {
    return <>invalid search</>;
  }

  const searchValue = canonicalSearch(origSearchValue);
  const data = await getDataForPieceType(piecetypeValue);
  if (
    !(searchValue in data.cases) &&
    !(invertSearchValue(searchValue) in data.cases)
  ) {
    return <>not found</>;
  }

  console.log(data.cases[searchValue]);
  return (
    <div>
      <div>
        <h3>{origSearchValue}</h3>
        {table(data, searchValue)}
      </div>
      <div>
        <h3>{invertSearchValue(origSearchValue)} (inverse)</h3>
        {table(data, invertSearchValue(searchValue))}
      </div>
    </div>
  );
}

const App = {
  view: () => (
    <div>
      <div>
        <label for="piecetype">Piece type</label>
        <select
          id="piecetype"
          size="8"
          oninput={(ev: InputEvent & { target: HTMLSelectElement }) => {
            piecetypeValue = ev.target.value;
          }}
        >
          <option value="Corner3Cycle">Corners</option>
          <option value="Edge3Cycle">Edges</option>
          <option value="Wing3Cycle">Wings</option>
          <option value="Midge3Cycle">Midges</option>
          <option value="XCenter3Cycle">X-centers</option>
          <option value="TCenter3Cycle">T-centers</option>
          <option value="LeftOblique3Cycle">
            Left obliques (e.g. Uf3l buffer)
          </option>
          <option value="RightOblique3Cycle">
            Right obliques (e.g. Uf3r buffer)
          </option>
        </select>
      </div>

      <div>
        <label for="search">Search</label>
        <input
          type="text"
          id="search"
          name="search"
          autofocus
          maxlength="3"
          placeholder="CAB"
          oninput={(ev: InputEvent & { target: HTMLInputElement }) => {
            searchInputValue = ev.target.value;
          }}
        />
      </div>

      {<div id="output">{doIt()}</div>}
    </div>
  ),
};

function spreadsheetNumberToLetter(x: number) {
  let s = "";
  while (x > 0) {
    s = String.fromCharCode("A".charCodeAt(0) + (x % 26)) + s;
    x /= 26;
    x = Math.floor(x);
  }
  return s;
}

function table(data: Data, search: string) {
  return (
    <table border={1}>
      <tbody>
        {Object.entries(data.cases[search]).map(([alg, variants]) => (
          <tr>
            <td>{alg}</td>
            <td>
              <div>
                {Object.entries(variants).map(([variant, sources]) => (
                  <div>{variant}</div>
                ))}
              </div>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function canonicalSearch(triple: string): string {
  let count = 0;
  while ((count++ < 10 && triple[1] < triple[0]) || triple[2] < triple[0]) {
    triple = [triple[1], triple[2], triple[0]].join("");
    count++;
  }
  return triple;
}

function invertSearchValue(triple: string): string {
  return [triple[0], triple[2], triple[1]].join("");
}
//
//search.addEventListener("input", update);
//piecetype.addEventListener("input", update);

m.mount(app, App);
