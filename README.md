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
| accuracy  | la precisione del risultato         | 0.001

[⚠️] Il parametro accuracy, incide molto sulla velocità di esecuzione del programma. Molto spesso sono sufficienti 3 cifre decimali, e sono un buon compromesso tra velocità di esecuzione e precisione del risultato.

L'output del programma permetterà di visualizzare l'integrale inserito dall'utente, un grafico che rappresenta la curva della funzione ed infine il risultato dell'operazione.

## Esempio di output
```
─────────────────────── COMPUTING ───────────────────────
                         1
                        ∫ x^3 dx
                         0
───────────────────────── GRAPH ────────────────────────
                         ⡁                       ⡎  0.9
                         ⠄                      ⡸  
                         ⠂                     ⢰⠁  
                         ⡁                    ⢠⠃   
                         ⠄                   ⢀⠎    
                         ⠂                  ⢀⠎     
                         ⡁                 ⢀⠎      
                         ⠄                ⢠⠊       
                         ⠂               ⡰⠁        
                         ⡁             ⡠⠊          
                         ⠄          ⢀⡠⠊            
                         ⠂      ⣀⡠⠤⠒⠁              
⠂⠐ ⠂⠐ ⠂⠐ ⠂⠐ ⠂⠐ ⢂⣐⠤⠖⠒⠒⠒⠒⠊⠋⡙⠉⠋⠙⠉⠋⠙ ⠂⠐ ⠂⠐ ⠂⠐ ⠂⠐ ⠂⠐ ⠂⠐ 
            ⢀⠔⠊⠁         ⠄                         
          ⢀⠔⠁            ⠂                         
        ⢀⠔⠁              ⡁                         
       ⡠⠃                ⠄                         
      ⡔⠁                 ⠂                         
     ⡜                   ⡁                         
    ⡜                    ⠄                         
   ⡜                     ⠂                         
  ⡰⠁                     ⡁                         
 ⢠⠃                      ⠄                         
⢀⠇                       ⠂                         
⡜                        ⡁                         
⠁                                                   -1.0
-1.0                                           1.0

──────────────────────── RESULT ────────────────────────
                         0.25
────────────────────────────────────────────────────────
```