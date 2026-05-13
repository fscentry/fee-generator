# Simple Fee Generator — Rust Implementation

A Implementation of Dynamic Fee Generator with Json input and result format.
---
## 🗂️ Project Background
This project was born from frustration with the limitations of existing systems and workflows. Over time, many ideas for improvements were difficult to realize because the current ecosystem was often rigid, overly complex, and not easy to collaborate with. As a result, experimenting with new approaches became harder than it should have been.

Another major challenge was communication and proposal discussions with large corporate environments. Bringing new technical ideas or alternative designs into established systems can be a very slow and complicated process, especially when decision-making layers are difficult to navigate. This repository became a way to independently explore those ideas without being blocked by organizational constraints.

The project is also a personal journey into learning and building with Rust, especially from the perspective of a developer transitioning from Java. Because of that, the codebase may reflect beginner-friendly patterns, exploration, experimentation, and gradual adaptation to Rust’s ownership model, safety principles, and performance-oriented mindset.

Of course, this repository is intentionally different from any internal or production implementation. Certain concepts, architectures, and workflows have been redesigned or simplified due to NDA and confidentiality restrictions. The goal of this project is not to replicate proprietary systems, but to create an open, educational, and community-friendly alternative inspired by real-world experience.

Hopefully, this repository can become:

- A learning reference for beginners moving from Java to Rust
- A place to experiment with system design ideas
- An open discussion space for improvements and collaboration
- A simpler and more transparent alternative approach

Contributions, suggestions, and constructive feedback are always welcome.


## 📌 Overview

This is base concept for dynamic calculation ( i will update later for more feature ):

- **1 Input e** — accepts json file input (in .txt) data separated by new line
- **1 Output** — produces a result (each line represent each data, set by unique key for identification)
- **Clustering** - you can define clustering data by its own characteristic base on rule filter
- **Sub Cluster** - you can also defining sub cluster if you want, if you do sub cluster must be reference to upper cluster for reference.
- **Dynamic lookup and Calculation** - this calculation run by dynamic lookup, witch mean you can using reference to own data. reference to constant, and set specific operation calculation without change code. all by set specific configuration (json)

## 📌 Feature Version Design ( Soon ).
- **Input** - input can be in real time ( via API and EventDriven like Kafka )
- **Output** -  output also can be set ( inserting to DB, response API, or another topic)
- **CLI** - program can be run as service and monitoring by cli ( status , log, load configuration, etc)
---


## ⚙️ Input

### `/config/configuration.toml` - project base setting

```toml
# configuration.toml
cluster_json_path = "./config/Cluster.json"
cluster_fee_json_path = "./config/FeeList.json"
cons_reference_path = "./config/Const.json"

#input file:
input_path = "./input/example.txt"
data_key = "trx_id"
## 0 for not backup
## 1 for backup ( moved into sepcific path )
backup = 0
backup_path = "./backup/"
# Output
output_path = "./output/"

```

### `Cluster - config/Cluster.json` Defining Cluster Rule
this is should follow base format, ( default should always defined)
```json
{
  "clusters": [
    {
      "name": "Normal Cluster",
      "id": "c-normal",
      "priority": 1,
      "sub_cluster": null,
      "rules": "{issuer} != {acquirer} && {issuer} != {destination} && {acquirer} != {destination} && {code} in ['200','404','201']"
    },
    {
      "name": "Default Cluster",
      "id": "default",
      "priority": 0,
      "sub_cluster" : null,
      "rules": null
    }
  ],
  "sub_clusters" :[
    {
      "name": "Normal Cluster - priority",
      "id": "c-normal-prio",
      "priority": 1,
      "sub_cluster": "c-normal",
      "rules": "{network} in [9000,8080,3306]"
    }
  ]
}
```
### `Fee - config/FeeList.json` Defining Fee Rule
- this is should follow base format, ( default should always defined)
- this defined each cluster how to be calculated, you can add specific calculation by adding rule
```json
[
  {
    "cluster_id": "c-normal",
    "rules": [
      {
        "name": "c-normal-transfer",
        "rule": "{code} == '02'",
        "calculation": [
          {
            "key": "Fee Issuer",
            "exp": "{amount} * 0.01"
          },
         ...
        ]
      },
      {
        ...
      }
    ]
  },
  {
    "cluster_id": "c-normal-prio",
    "rules": [
      {
        "name": "c-normal-prio-transfer",
        "rule": "{code} == '02'",
        "calculation": [
          {
            "key": "Fee Issuer",
            "exp": "{amount} * 0.01"
          },
          ...
        ]
      },
      ...
    ]
  },
  {
    "cluster_id": "default",
    "rules": [
      {
        "name": "all trx",
        "rule": null,
        "calculation": [
          ...
        ]
      }
    ]
  }
]
```
### `Const - config/Const.json` Defining Constat Reference
- this is should follow base format.
- this defined const reference and can be used across Cluster, Fee, and Input using tag @ + [name_of const] exp @PPN
```json
{
  "visa": "'VISA'",
  "PPN": 0.12,
  "fee_iss_percentage": 0.02,
  "fee_acq_percentage": 0.01,
  "fee_dest_percentage": 0.027,
  "tax_2026" : 0.12
}
```
### `Input - input/example.txt` Defining Constat Reference
- this is json format, each transaction is defined and separated by row
```txt
{"trx_id": "10001", "from": "Visa","acquirer" : "MC", "destination" : "-", "code" : "'01'", "network" : 4040, "amount" : 20000}
...
{"trx_id": "10003", "issuer": "Visa","acquirer" : "UP", "destination" : "-", "code" : "'01'", "network" : 8080, "amount" : 25000}
```



---
## 📤 Output

After the training process, the program will display:

| Output   | Description                                                                                             |
|----------|---------------------------------------------------------------------------------------------------------|
| `Backup` | file will backupd after calculated ( you can turn it off in configuration )                             |
| `output` | output will in .txt file, each row is a json string that show output fee and identified by **data_key** |

Example output:

```
{"Fee Acquirer":"200","Fee Destination":"540","Fee Issuer":"400","Tax Fee Issuer":"48","data_key":"10001"}
...
{"Fee Acquirer":"250","Fee Destination":"675","Fee Issuer":"500","Tax Fee Issuer":"60","data_key":"10003"}
```

---

## 🚀 How to Run

Make sure [Rust and Cargo](https://www.rust-lang.org/tools/install) are installed on your machine.

### Run the application

```bash
cargo run
```

### Run and save output to a file

```bash
cargo run > output.txt
```

> The `output.txt` file will contain the full training history log and final prediction results.

---

## 🧮 How It Works

```
Input (x)
    |
Clusterize  ----> have a sub-cluster ?  ---> finding again
    |                                              |
    |  <-------------------------------------------|
    |
Find Fee by Cluster
    |
 Calculate Fee
    |  
 Generate Output
 
```

---

## 🛠️ Dependencies

This project requires no external dependencies. It uses only the **Rust standard library**.

```toml
[package]
name = "fee_generator"
version = "0.1.0"
edition = "2024"

[dependencies]
evaluator_rs = "0.1.5"
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.149"
toml = "1.1.2"
regex = "1.12.3"
derivative = "2.2.0"

```

---
