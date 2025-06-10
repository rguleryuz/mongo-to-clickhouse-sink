# mongo-to-clickhouse-sink

A small Rust application that watches a MongoDB collection and replicates new
inserts into a ClickHouse table. Documents from MongoDB are stored in the
ClickHouse table as JSON strings.

## Building

```bash
cargo build --release
```

## Configuration

The application is configured via environment variables:

- `MONGODB_URI` - connection string to the MongoDB instance
- `MONGODB_DB` - database name
- `MONGODB_COLL` - collection name to watch
- `CLICKHOUSE_URL` - base URL of the ClickHouse server
- `CLICKHOUSE_DATABASE` - ClickHouse database name
- `CLICKHOUSE_TABLE` - table to insert replicated rows into

The ClickHouse table should contain at least one `String` column named `data`:

```sql
CREATE TABLE my_table (data String) ENGINE = MergeTree() ORDER BY tuple();
```

Run the application after setting the environment variables:

```bash
MONGODB_URI="mongodb://localhost:27017" \
MONGODB_DB=mydb MONGODB_COLL=mycoll \
CLICKHOUSE_URL="http://localhost:8123" \
CLICKHOUSE_DATABASE=default CLICKHOUSE_TABLE=my_table \
./target/release/mongo-to-clickhouse-sink
```
