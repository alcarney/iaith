import './styles.css'

const rust = import('./pkg');

const runButton = document.getElementById("run")
const input = document.getElementById("input")
const output = document.getElementById("output")
const tape = document.getElementById("tape")

function setCell(index, value) {
  let cell = tape.querySelector(`[data-index="${index}"]`)
  if (!cell) {
    cell = document.createElement("div")
    cell.setAttribute("data-index", index)
    cell.style.placeItems = "center"
    cell.className = "grid flex-shrink-0 w-12 h-12 p-2 bg-white border"

    tape.append(cell)
  }

  cell.innerText = `${value}`
}

for (let i = 0; i < 64; i++) {
  //  setCell(i, 0)
}

rust
  .then(mod => {
    runButton.addEventListener('click', event => {
      let source = input.value
      let result = mod.execute(source)
      output.textContent = result
    })

  })
  .catch(console.error);
