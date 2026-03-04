---
### Note : 

la meilleur complexité possible est O (1) qui reviens a dire le “constant time” ou “constant space” ce qui veut dire que l’algorithme utilise toujours la même ressources quelques soit la quantité de l’entrer. Cela ne veut pas dire qu’il sera rapide cela veut juste dire sa durée d'exécution est indépendante de la taille de l'entrée.


### tips :

Prefix sum est une forme de pre traitement a utiliser dans un tableau pour calculer tout les elements avant l'index i. En general on parcour un tableau pour calculer un tableau avec la formule suivante : `prefix[j] - prefix[i] + tab[i]` ou `prefix[j] + prefix[i - 1]`
Le prétraitement est une stratégie utile dans divers problèmes où nous stockons des données précalculées dans une structure de données avant d'exécuter la logique principale de notre algorithme.

Approche brute force : approche d'une complexiter algorithmique de 0(n^2) car pour chaque indexe on calculera on iterera de 0 vers i pour trouver la somme de la section gauche, puis meme iteration mais cette fois ci de i+1 vers la fin du tableau pour trouver la somme de la section droite.
---
