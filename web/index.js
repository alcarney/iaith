import './styles.css'
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'

const rust = import('./pkg');

const html = document.querySelector("html")
const runButton = document.getElementById("run")

const inputHeader = document.getElementById("input-header")
const input = document.getElementById('input')

const outputHeader = document.getElementById("output-header")
const output = document.getElementById("output")

const footer = document.querySelector("footer")

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

layout()
window.onresize = layout

rust
  .then(mod => {
    runButton.addEventListener('click', event => {
      let source = editor.getValue() //input.value
      let result = mod.execute(source)
      output.textContent = result
    })

  })
  .catch(console.error);
