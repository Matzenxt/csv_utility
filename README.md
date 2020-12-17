# CSV Utility


## About
[![Crates.io](https://img.shields.io/crates/v/csv_utility.svg)](https://crates.io/crates/csv_utility)


## Usage


### Installation
Install ``csv_utility`` via cargo with the following command

```sh
cargo install csv_utility
```

or build it yourself by pulling the repo.

### Using CSV Utility from the Command Line

```sh
Name
        csv_utility

Author:
        Matthias Lodner <matthias.lodner@uni-ulm.de>

Description:
        csv_utility command [flags]

Flags:
        -s, --source <string>      : path to source file
        -d, --destination <string> : path to destination file
        -o, --output <string>      : path to output file
        -m, --mappings <string>    : path to mappings file
        -t, --threshold <string>   : threshold, a positiv number
        -h, --help                 : Show help

Commands:
        m, map     : Maps entries from source file to header from header file and saves to output file
        rer, rmer  : Removes empty rows from source csv file and saves to output file
        rrwt, rmwt : Remove rows with less than --threshold entries from source csv file and saves to output file
        a, append  : Appends two csv files with the same header line
        s, stats   : Shows some stats about the content of the csv file

Version:
        0.1.0
```

### Examples
In the following examples, the table below is used as the ``source.csv`` file.

|       city       	| size    	| population 	|
|:----------------:	|:---------:|:-----------:	|
|   New York City  	|  783,8  	|    8,399   	|
|                   |           |               |
| Washington, D.C. 	|   177   	|   705.749  	|
|        Texas      |           |  29.000.000   |
|    California    	| 423.970 	|    39,51   	|
|                   |           |               |
| Los Angeles      	| 1.290,6 	|  3.979.576 	|
|  San Francisco    |           |               |
|                   |           |               |

#### Map
In this example the content of ``source.csv`` gets mapped to the following header from ``dest.csv`` and saved to ``out.csv``.

|       stadt     	| einwohner	    | fläche    |
|:----------------:	|:-----------:  |:---------:|

```sh
csv_utility map -s source.csv -d dest.csv -o out.csv
```

Running the command above opens a user interface with the following options:
- ``Map``
  <br> Let you map the columns from the source file to the destination file.
- ``Save mapping file``
  <br> Saves the mapping created in ``Map``.
- ``Save as new mapping file``
  <br> Saves the mapping created in ``Map`` to a new mapping file.
- ``Cancel``
  <br> Cancels the command.
- ``Save and exit``
  <br> Saves the columns according to the mapping in the output file and exit afterwards.

The result looks like

|       stadt      	| einwohner 	| fläche   	|
|:----------------:	|:-----------:	|:---------:|
|   New York City  	|    8,399   	|  783,8  	|
|                   |               |           |
| Washington, D.C. 	|   705.749  	|   177   	|
|        Texas      |  29.000.000   |           |
|    California    	|    39,51   	| 423.970 	|
|                   |               |           |
| Los Angeles      	|  3.979.576 	| 1.290,6 	|
|  San Francisco    |               |           |
|                   |               |           |

#### rmer
Running the following command on ``source.csv``

```sh
csv_utility rmer -s source.csv -o out.csv
```

Gets us ``out.csv`` with the following content:

|       city       	| size    	| population 	|
|:----------------:	|:---------:|:-----------:	|
|   New York City  	|  783,8  	|    8,399   	|
| Washington, D.C. 	|   177   	|   705.749  	|
|        Texas      |           |  29.000.000   |
|    California    	| 423.970 	|    39,51   	|
| Los Angeles      	| 1.290,6 	|  3.979.576 	|
|  San Francisco    |           |               |

#### rmwt
Running the following command on ``source.csv``

```sh
csv_utility rmwt -s source.csv -o out.csv -t 1
```

Gets us ``out.csv`` with the following content:

|       city       	| size    	| population 	|
|:----------------:	|:---------:|:-----------:	|
|   New York City  	|  783,8  	|    8,399   	|
| Washington, D.C. 	|   177   	|   705.749  	|
|        Texas      |           |  29.000.000   |
|    California    	| 423.970 	|    39,51   	|
| Los Angeles      	| 1.290,6 	|  3.979.576 	|

Using the 2 for ``-t`` the output file would look like this.

|       city       	| size    	| population 	|
|:----------------:	|:---------:|:-----------:	|
|   New York City  	|  783,8  	|    8,399   	|
| Washington, D.C. 	|   177   	|   705.749  	|
|    California    	| 423.970 	|    39,51   	|
| Los Angeles      	| 1.290,6 	|  3.979.576 	|

#### append
With append the ``-d`` file get appended to the ``-s`` file and written to the output file.

```sh
csv_utility append -s source.csv -d source.csv -o out.csv
```

|       city       	| size    	| population 	|
|:----------------:	|:---------:|:-----------:	|
|   New York City  	|  783,8  	|    8,399   	|
|                   |           |               |
| Washington, D.C. 	|   177   	|   705.749  	|
|        Texas      |           |  29.000.000   |
|    California    	| 423.970 	|    39,51   	|
|                   |           |               |
| Los Angeles      	| 1.290,6 	|  3.979.576 	|
|  San Francisco    |           |               |
|                   |           |               |
|   New York City  	|  783,8  	|    8,399   	|
|                   |           |               |
| Washington, D.C. 	|   177   	|   705.749  	|
|        Texas      |           |  29.000.000   |
|    California    	| 423.970 	|    39,51   	|
|                   |           |               |
| Los Angeles      	| 1.290,6 	|  3.979.576 	|
|  San Francisco    |           |               |
|                   |           |               |

#### stats
The Stats command shows some information about the content of the csv file.

```sh
csv_utility append -s source.csv -d source.csv -o out.csv

Stats for in.csv
  - 3 columns
  - 9 lines total
  - 4 full lines
  - 2 partly full lines
  - 3 empty lines
```
