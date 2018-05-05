Código usado no artigo ["Uma Busca Local Iterada para o Problema da Árvore Geradora
Mínima sob Restrições de Conflitos"](http://www.sbpo2017.iltc.br/pdf/169115.pdf), publicado nos
anais do XLIX SBPO, 2017.


## Modo de uso

Para compilar o programar é necessário o [Rust](https://www.rust-lang.org) versão 1.21.0 ou superior.

```sh
cargo build --release
```

Os resultados para a configuração B1 foram obtidos executando o comando:

```sh
target/release/mstcc --ils-max-iters m --ils-excludes 3 random ils-2ex arquivo-instancia
```

Os resultados para a configuração B2 foram obtidos executando o comando:

```sh
target/release/mstcc --ils-max-iters m --ils-excludes 3 random ils-4ex arquivo-instancia
```

Onde `m` é número de arestas da instância.

Veja o modo de uso e todas as opções do programa executando:

```sh
target/release/mstcc --help
```


## Instâncias

As instâncias podem ser obtidas enviando um email para os autores do artigo.


## Changelog

Mudanças deste a publicação do artigo:

- Atualização das dependências (versão mínima do Rust foi alterada para 1.21.0)
- Formatação do código com rustfmt
- Corrigidos alguns avisos do clippy


## Licença

- [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/)
