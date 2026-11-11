# Algoritmo A* (em desenvolvimento)

Esse projeto tem como finalidade comparar implementações do algoritmo `A*` popularmente utilizado para encontrar caminhos. Se trata de um projeto da disciplina de grafos da `UFRN`.

## Instruções de uso

Considerando que você tem o `cargo v1.99.0` instalado:

```sh
cargo run -- -f graphs/a.txt
```

Alternativamente, para roda a versão concorrente do algoritmo:

```sh
cargo run -- -cf graphs/a.txt
```

Caso queira rodar os exemplos considerando os nós `_` como água, basta executar:

```sh
cargo run -- -wf graphs/a.txt
```

O padrão é converter os nós de água em parede. Os nós de água custam o dobro para serem visitados e para visitar outro nó.

Em `/graphs` você encontrará os demais arquivos guardando os grafos que deixamos prontos para serem utilizados.

Para rodar o código em modo de produção (ou seja, o modo performático):

```sh
cargo run --release -- -f graphs/a.txt
```

Para a lista completa de comandos:

```sh
cargo run -- -h
```

## Instruções para desenvolvedores

Se você utiliza o `VSCode` ou `Antigravity IDE`, é recomendado utilizar a extensão `rust-analyzer`.