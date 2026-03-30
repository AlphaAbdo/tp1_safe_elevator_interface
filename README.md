# Etudiant
 Abdelkader Kantaoui

# TP 1 - Simulateur d'Ascenseur

### Sujet

Dans ce TP, vous allez implémenter un simulateur d'ascenseur respectant un modèle métier précis. L'ascenseur doit gérer son état, sa position, et une file de destinations, tout en respectant les règles métier définies.

### Modèle métier à respecter

- Un ascenseur maintient:
  - un étage courant,
  - un état courant,
  - une file de destinations.
- États possibles:
  - `Idle` (immobile),
  - `MovingUp` (en montée),
  - `MovingDown` (en descente),
  - `DoorsOpen` (portes ouvertes).
- Erreurs métier possibles:
  - `InvalidFloor(floor)`,
  - `DoorsAlreadyOpen`,
  - `DoorsAlreadyClosed`,
  - `CannotOpenWhileMoving`,
  - `CannotMoveDoorsOpen`,
  - `EmptyQueue`.

### Opérations attendues

- Initialiser l'ascenseur à un étage de départ.
- Lire l'état courant: étage, état, file d'attente.
- Ajouter un appel vers un étage.
- Ouvrir les portes.
- Fermer les portes.
- Avancer l'ascenseur d'un pas (`step`).

### Règles métier

1. Les étages autorisés sont `0` à `5` inclus.
1. Initialisation ou appel hors plage -> `InvalidFloor(floor)`.
1. Un appel vers l'étage courant est ignoré.
1. Un appel déjà présent dans la file est ignoré.
1. Si l'ascenseur est `Idle` et qu'une destination est ajoutée, il prend immédiatement une direction cohérente.
1. Ouvrir les portes alors qu'elles sont déjà ouvertes -> `DoorsAlreadyOpen`.
1. Ouvrir les portes pendant un mouvement -> `CannotOpenWhileMoving`.
1. Fermer les portes alors qu'elles sont déjà fermées -> `DoorsAlreadyClosed`.
1. Fermer les portes avec file vide remet l'ascenseur à `Idle`.
1. Fermer les portes avec file non vide remet l'ascenseur en mouvement vers la prochaine destination.
1. `step` portes ouvertes -> `CannotMoveDoorsOpen`.
1. `step` sans destination -> `EmptyQueue` et état `Idle`.
1. `step` avec destination déplace l'ascenseur d'un seul étage vers cette destination.
1. À l'arrivée sur la destination courante: retrait de la destination de la file et passage en `DoorsOpen`.

### Validation

- `cargo test` doit réussir sans erreurs.
- `cargo check` doit réussir sans erreurs.
- `cargo clippy` doit signaler zéro avertissement.
- `cargo fmt` doit indiquer que le code est correctement formaté.
