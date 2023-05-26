package main

import (
	"context"
	"fmt"
	"github.com/redis/go-redis/v9"
	log "github.com/sirupsen/logrus"
	"os"
	"time"
)

var (
	REDIS_HOST = os.Getenv("REDIS_HOSTNAME")
	ctx        = context.Background()
)

func init() {
	if REDIS_HOST == "" {
		REDIS_HOST = "localhost"
	}
}

func main() {
	rdb := redis.NewClient(&redis.Options{
		Addr:     fmt.Sprintf("%s:6379", REDIS_HOST),
		Password: "", // no password set
		DB:       0,  // use default DB
	})
	log.Info("starting main loop...")
	for {

		result, err := rdb.ZRangeByScoreWithScores(ctx, "buy_list", &redis.ZRangeBy{
			Min: "-inf",
			Max: "+inf",
		}).Result()

		if err != nil {
			log.Fatal(err)
		}

		log.Infof("found %d items in buy_list", len(result))

		// Process the result
		for _, z := range result {
			ticker := z.Member.(string)
			timestamp := time.Unix(int64(z.Score), 0).UTC()
			fmt.Printf("Ticker: %s, Buy at: %s\n", ticker, timestamp.Format("2006-01-02 03:04PM"))
			if timestamp.Before(time.Now().UTC()) {
				log.Infof("sending a buy order for %s", ticker)
			}
		}
		time.Sleep(1 * time.Minute)
	}
}
