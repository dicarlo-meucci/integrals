# Integrals

## Descrizione
Questo software permette di calcolare con una certa precisione il valore dell'area sottostante una curva, espressa da una funzione f(x) negli intervalli specificati <strong>a</strong> e <strong>b</strong>. Tutti questi parametri sono inseriti dall'utente, che potrà visualizzare il risultato approssimativo sullo schermo del terminale

## Istruzioni
All'avvio del programma, esso chiederà all'utente i valori da inserire.

| Parametro |     Significato    | Esempio
|:---------:|:------------------:|:--------:|
| f(x)      | la funzione matematica da integrare | 3*x^3
| a         | l'estremo minore                    | 0
| b         | l'estremo maggiore                  | 3
| precision | la precisione del risultato         | 0.001
| algorithm | l'algoritmo da utilizzare           | 1

[⚠️] Il parametro precision, incide molto sulla velocità di esecuzione del programma. Molto spesso sono sufficienti 3 cifre decimali, e sono un buon compromesso tra velocità di esecuzione e precisione del risultato.

L'output del programma permetterà di visualizzare l'integrale inserito dall'utente, un grafico che rappresenta la curva della funzione ed infine il risultato dell'operazione.

## Flags
Il programma supporta anche l'inserimento tramite flags

`integrals --help`

```
Usage: integrals [OPTIONS]

Options:
  -f, --function <FUNCTION> // la funzione da calcolare
  -u, --upper <UPPER> // l'estremo maggiore
  -l, --lower <LOWER> // l'estremo minore    
  -p, --precision <PRECISION> // la precisione del calcolo
  -a, --algorithm <ALGORITHM> // l'algoritmo da utilizzare (1-3)
  -d, --decimals <DECIMALS> // il numero di cifre decimali da mostrare
  -h, --help // mostra il messaggio d'aiuto
```

## Esempio di output
`integrals --function x^3 --lower 0 --upper 1 --precision 0.001 --algorithm 1 --decimals 10`

```
──────────────────────── COMPUTING ────────────────────────
                         1
                        ∫ x^3 dx
                         0
──────────────────────── ALGORITHM ────────────────────────

                         Rectangles

─────────────────────────── GRAPH ──────────────────────────
                              ⡁                           ⢠⠊  950.8
                              ⠄                          ⡰⠁  
                              ⠂                        ⡠⠊    
                              ⡁                      ⡠⠊      
                              ⠄                   ⢀⡠⠊        
                              ⠂                ⢀⡠⠒⠁          
                              ⡁           ⣀⡠⠤⠒⠊⠁             
⠄⠠ ⠄⠠ ⠄⠠ ⠄⠠ ⠄⠠ ⠄⠠⢀⣄⣠⠤⠤⠤⠤⠖⠲⠒⠖⠲⠒⠖⠲⠒⠖⠲⠒⠖⠲⠒⠍⠩⠉⠄⠠ ⠄⠠ ⠄⠠ ⠄⠠ ⠄⠠ ⠄⠠ ⠄
            ⢀⡠⠔⠒⠉⠁            ⠂                              
         ⢀⠤⠊⠁                 ⡁                              
       ⡠⠒⠁                    ⠄                              
     ⡠⠊                       ⠂                              
   ⡠⠊                         ⡁                              
 ⢀⠔⠁                          ⠄                              
⡠⠊                            ⠂                              
⠁                             ⠁                               -1000.0
-10.0                                                    10.0

───────────────────────── RESULT ─────────────────────────
                         0.2505002500
──────────────────────────────────────────────────────────

```

## Test
Per eseguire i test è necessario avere <strong>Rust</strong> installato

```
cargo test
```

```
running 4 tests
test tests::tests::math_function_evaluation ... ok
test tests::tests::math_trapezoid_integration ... ok
test tests::tests::math_rectangles_integration ... ok
test tests::tests::math_parabola_integration ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

## Funzionalità pianificate
- Esecuzione parallela (multithread)