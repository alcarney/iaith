import './styles.css'
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'

const rust = import('./pkg');

const footer = document.querySelector("footer")
const html = document.querySelector("html")
const input = document.getElementById('input')
const inputHeader = document.getElementById("input-header")
const output = document.getElementById("output")
const outputHeader = document.getElementById("output-header")
const runButton = document.getElementById("run")
const tape = document.getElementById("tape")

let editor = monaco.editor.create(input, {
  value: 'hello, world',
});

function layout() {

  const WIDTH = html.clientWidth
  const HEIGHT = html.clientHeight

  let inputBbox = input.getBoundingClientRect()
  let inputHeaderBbox = inputHeader.getBoundingClientRect()

  const editorWidth = Math.floor(WIDTH * 0.6)
  const editorHeight = HEIGHT - inputBbox.top - footer.clientHeight

  inputHeader.style.width = `${editorWidth}px`
  editor.layout({ width: editorWidth, height: editorHeight })

  outputHeader.style.position = "absolute"
  outputHeader.style.top = `${inputHeaderBbox.top}px`
  outputHeader.style.left = `${editorWidth}px`
  outputHeader.style.width = `${WIDTH - editorWidth}px`

  output.style.position = `absolute`
  output.style.top = `${inputBbox.top}px`
  output.style.left = `${editorWidth}px`
  output.style.width = `${WIDTH - editorWidth}px`
  output.style.height = `${editorHeight}px`
}

function setTape(memory, pointer) {
  let classes = "grid flex-shrink-0 w-12 h-12 p-2 bg-white place-center"
  for (let i = 0; i < memory.length; i++) {
    let cell = tape.querySelector(`[data-index="${i}"]`)
    if (!cell) {
      cell = document.createElement("div")
      cell.setAttribute("data-index", `${i}`)
      tape.append(cell)
    }

    if (pointer === i) {
      classes += " border-2 border-gray-600"
    } else {
      classes += " border"
    }

    cell.className = classes
    cell.innerText = `${memory[i]}`
  }
}

layout()
window.onresize = layout

rust
  .then(mod => {
    runButton.addEventListener('click', event => {
      let source = editor.getValue()
      let result = mod.execute(source)

      output.innerText = result.stdout
      setTape(result.memory, result.pointer)
    })

  })
  .catch(console.error);
