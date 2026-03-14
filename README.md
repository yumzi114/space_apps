# space_apps
## This repository used [spacetimeDB](https://github.com/clockworklabs/SpacetimeDB)


Please refer to the example code in the official [documentation](https://spacetimedb.com/docs/tutorials/chat-app).
## Template Structure 
| Template  | Discliption |
| ------------- |:-------------:|
| db-schema      | SpacetimeDB schema     |
| api_server      | Axum api server     |
| client_ui      | egui client     |
| eframe_template      | WASM Template     |


## SpacetimeDB Commands

```bash
# Reset Database
spacetime delete --server proxmox-stdb eomdb
spacetime publish --server proxmox-stdb --module-path spacetimedb eomdb

# Run SQL Query
spacetime sql --server proxmox-stdb eomdb "SELECT * FROM user"

# Apply Database Changes
spacetime publish --server proxmox-stdb --module-path spacetimedb eomdb

# Generate Rust Bindings
spacetime generate --lang rust --out-dir src/module_bindings --module-path ../db-schema/spacetimedb

# Monitor Database Logs
spacetime logs --server proxmox-stdb --follow eomdb
```
