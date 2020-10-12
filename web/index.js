import './styles.css'
const rust = import('./pkg');

const runButton = document.getElementById("run")
const input = document.getElementById("input")
const output = document.getElementById("output")

rust
  .then(mod => {
    runButton.addEventListener('click', event => {
      let source = input.value
      let result = mod.execute(source)
      output.value = result
    })

  })
  .catch(console.error);
