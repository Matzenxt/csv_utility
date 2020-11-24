# CSV Utility


## About


## Usage


### Installation


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
| Washington, D.C. 	|   177   	|   705.749  	|
|    California    	| 423.970 	|    39,51   	|
| Los Angeles      	| 1.290,6 	|  3.979.576 	|

#### Map
In this example the content of ``source.csv`` gets mapped to the following header from ``dest.csv`` and saved to ``out.csv``.

|       city       	| size    	| population 	|
|:----------------:	|:---------:|:-----------:	|

