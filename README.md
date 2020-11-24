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

