# Hello World - Rust WebAssembly

Este é um repositório de estudos sobre implementações `WASM` com `Rust`.

O projeto é originado do exemplo de iniciante do <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>, onde realizarei testes e implementações para estudo.

## Build WASM

Ao executar o comando de build:

```
wasm-pack build
```

Temos um novo diretório chamado `pkg` contendo os resultados da compilação, que inclusive traz um `package.json`, o `wasm` e o `.js` com a importação do pacote.

## Implementando como pacote

No diretório `www`, há toda estrutura para importação do pkg como um módulo e a execução no browser.

```
cd www
npm i
npm run start
```

## Comentários

Durante o desenvolvimento, precisei apenas rodar novamente o `wasm-pack build` pois o `npm run start` do www já identifica na hora a mudança na chamada do método.

Para ficar lindo, seria bom um `cargo watch` e ficará bem prático para desenvolver, mas ainda não consegui então por enquanto um simples `inotifywait` do linux já nos ajudará:

```
while inotifywait -e close_write *; do wasm-pack build; done
```

## Teste 1 - Função Soma

Função para calcular soma, é bem simples mas já dá um exemplo de como passar parâmetro do browser para a função escrita em Rust.

## Teste 2 - Array 10M posições

Criei dois botões onde cada um deles faz a mesma coisa, cria um array e, dentro de um loop, o popula com 10 milhões de posições. Porém um é feito em Rust e outro em JS. É importante ressaltar que não há nada de avançado e o teste nos dá clareza sobre a performance com WASM mas também há uma incôgnita em relação a forma de implementação das estruturas de dados internas em cada linguagem.

Veja como ficou a performance aqui:

```
Tempo com WASM: 302.65500000314205 ms
Tempo com JS: 188.13500000032946 ms

Tempo com WASM: 78.50500000131433 ms
Tempo com JS: 176.22499999924912 ms

Tempo com WASM: 75.67500000004657 ms
Tempo com JS: 147.85499999925378 ms

Tempo com WASM: 76.91499999782536 ms
Tempo com JS: 165.16500000216183 ms

Tempo com WASM: 76.1899999997695 ms
Tempo com JS: 169.28999999799998 ms

Tempo com WASM: 66.44500000038533 ms
Tempo com JS: 241.32000000099652 ms

Tempo com WASM: 75.53499999994528 ms
Tempo com JS: 172.33499999929336 ms

Tempo com WASM: 77.49999999941792 ms
Tempo com JS: 170.4199999985576 ms

Tempo com WASM: 75.49999999901047 ms
Tempo com JS: 156.67499999835854 ms

Tempo com WASM: 78.80999999906635 ms
Tempo com JS: 171.71999999845866 ms
```

Note que a primeira execução com Rust demora MUITO, em contra-partida, todas as próximas não passam de 80ms enquanto com JS nenhuma é inferior a 150ms nunca.
Eu suponho que esta primeira execução seja lenta assim pois o binário ainda não foi carregado para a engine do navegador, deve haver alguma forma de melhorar isso para funções principais. 

A performance é fantástica ao meu ver, me pergunto por que não fazemos todas as operações básicas do JS com WASM ainda. Este foi um teste simples, mas imagine como seria rápida se as principais operações fossem realizadas assim.