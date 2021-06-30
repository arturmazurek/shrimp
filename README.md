# shrimp
Rust file server based on Rocket

GET /files - list of filenames and their checksums<br/>
GET /file/metadata/<checksum> - metadata for a specific file<br/>
GET /file/<checksum> - downloads a specific file<br/>

Checksums are SHA256
