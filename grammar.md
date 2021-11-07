# Grammar

Grammar is a set of rules that describe the structure of a language. Ventax grammar is generated using `PEG` method and pest parser.


This table will be updated as the grammar grows.
| name      | syntax          | usage              |
|-----------|-----------------|--------------------|
| Number    | /d+,([-]/d+)    | 3,(-15)            |
| Operation | '+','-','*','/' | Number `+` Number  |
|           |                 |                    |
