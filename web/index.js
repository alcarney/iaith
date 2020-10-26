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

monaco.languages.register({ id: 'brainf*ck' })
monaco.languages.setMonarchTokensProvider('brainf*ck', {
  defaultToken: 'comment.doc',
  tokenizer: {
    root: [
      [/[\[\]]/, 'constant'],
      [/[<>]/, 'type.identifier'],
      [/[+-]/, 'number.float'],
      [/[.,]/, 'operator']
    ]
  }
})

let example = `\
Example "Hello World!" program as found on the EsoLang wiki
https://esolangs.org/wiki/Brainfuck#Hello.2C_World.21

+++++ +++               Set Cell #0 to 8
[
    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   as the cell will be cleared by the loop
        >++             Add 4*2 to Cell #2
        >+++            Add 4*3 to Cell #3
        >+++            Add 4*3 to Cell #4
        >+              Add 4 to Cell #5
        <<<<-           Decrement the loop counter in Cell #1
    ]                   Loop till Cell #1 is zero
    >+                  Add 1 to Cell #2
    >+                  Add 1 to Cell #3
    >-                  Subtract 1 from Cell #4
    >>+                 Add 1 to Cell #6
    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
    <-                  Decrement the loop Counter in Cell #0
]                       Loop till Cell #0 is zero

The result of this is:
Cell No :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^

>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++ ++..+++.          Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.----- -.----- ---.  Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6`

let editor = monaco.editor.create(input, {
  language: 'brainf*ck',
  value: example,
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
  for (let i = 0; i < memory.length; i++) {
    let classes = "grid flex-shrink-0 w-12 h-12 p-2 bg-white place-items-center"

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
