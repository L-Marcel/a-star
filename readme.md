# Algoritmo A*

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

Em `/graphs` você encontrará os demais arquivos guardando os grafos que deixamos prontos para serem utilizados.

Para rodar o código em modo de produção (ou seja, o modo performático):

```sh
cargo run --release -- -f graphs/a.txt
```

Para a lista completa de comandos:

```sh
cargo run -- -h
```