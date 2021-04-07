import * as wasm from "wasm-tests";

document.getElementById('somarComWasm').addEventListener('click', () => {
    const number1 = document.getElementById('number1').value
    const number2 = document.getElementById('number2').value
  
    wasm.calculate(number1, number2)
})

document.getElementById('arrayWASM').addEventListener('click', () => {
    const start = window.performance.now();

    wasm.make_array(10000000)

    const end = window.performance.now();
    console.log(`Tempo com WASM: ${end - start} ms`);
})

document.getElementById('arrayJS').addEventListener('click', () => {
    const start = window.performance.now();
    
    const array = []
    const lenght = 10000000;

    for (let i = 0; i < lenght;i++) {
        array.push(i);
    }

    const end = window.performance.now();

    console.log(`Tempo com JS: ${end - start} ms`);
})