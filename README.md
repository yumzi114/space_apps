# space_apps

쿼리문 
spacetime sql --server proxmox-stdb eomdb "SELECT * FROM user"

디비반영
spacetime publish --server proxmox-stdb --module-path spacetimedb eomdb


바인딩생성
spacetime generate --lang rust --out-dir src/module_bindings --module-path ../db-schema/spacetimedb


디비 모니터링
spacetime logs --server proxmox-stdb --follow eomdb