# Recipe Search 2m Ingestion

This program parse the CSV file and store the data in a PostgreSQL database.

## Usage

```
Ingests the 2m recipe dataset into the database

Usage: recipe-search-2m-ingestion.exe [OPTIONS]

Options:
  -c, --clear
  -n, --no-insert      
  -l, --limit <LIMIT>
  -h, --help           Print help
```

## Dataset

Download the dataset from the release page, then put `full_dataset.csv` into `./dataset` folder.

## Source

Dataset is from RecipeNLG.

```
@inproceedings{bien-etal-2020-recipenlg,
    title = "{R}ecipe{NLG}: A Cooking Recipes Dataset for Semi-Structured Text Generation",
    author = "Bie{\'n}, Micha{\l}  and
        Gilski, Micha{\l}  and
        Maciejewska, Martyna  and
        Taisner, Wojciech  and
        Wisniewski, Dawid  and
        Lawrynowicz, Agnieszka",
    booktitle = "Proceedings of the 13th International Conference on Natural Language Generation",
    month = dec,
    year = "2020",
    address = "Dublin, Ireland",
    publisher = "Association for Computational Linguistics",
    url = "https://www.aclweb.org/anthology/2020.inlg-1.4",
    pages = "22--28"
}
```