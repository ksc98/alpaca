# Alpaca

## Instructions

```
gh repo clone ksc98/alpaca
cd alpaca
./scripts/start_dragonfly.sh
./scripts/populate_redis.sh
docker-compose up -d engine
cd engine/apctl
cargo r -- -t GOOGL
docker-compose logs -f engine
```