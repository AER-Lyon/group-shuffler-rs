# Group Shuffler made in rust

Un outil en ligne de commande pour générer des équipes à partir de fichiers CSV de logins ou de fichiers d'équipes existants.

## Fonctionnalités

- Traitement de fichiers CSV contenant des listes de logins
- Traitement de fichiers d'équipes existants au format CSV
- Génération aléatoire d'équipes à partir des données d'entrée
- Création automatique d'un fichier CSV de sortie avec horodatage
- Support de différents délimiteurs pour les fichiers CSV

## Installation

Assurez-vous d'avoir Rust et Cargo installés, puis clonez le dépôt et compilez le projet :

```bash
git clone git@github.com:AER-Lyon/group-shuffler-rs.git
cd group-shuffler-rs
cargo build --release
```

L'exécutable compilé se trouvera dans `target/release/group-shuffler-rs`.

## Utilisation

```bash
group-shuffler-rs --filename FICHIER [--delim DELIMITEUR]
```

### Arguments

- `--filename`, `-f` : Chemin vers le fichier CSV à traiter (obligatoire)
- `--delim`, `-d` : Délimiteur à utiliser pour le parsing CSV (par défaut : ';')

### Exemples

Traiter un fichier de logins avec le délimiteur par défaut :
```bash
group-shuffler-rs --filename logins.csv
```

Traiter un fichier d'équipes existant avec un délimiteur personnalisé :
```bash
group-shuffler-rs --filename equipes.csv --delim ','
```
