package main

import (
	"bytes"
	"compress/gzip"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"regexp"
	"sync"
	"time"

	"github.com/gin-gonic/gin"
)

type lazyLoader struct {
	mathLoaded   sync.Once
	imageLoaded  sync.Once
}

var loader = &lazyLoader{}

func main() {
	r := gin.Default()

	// Middleware to calculate execution time
	r.Use(func(c *gin.Context) {
		lambdaStart := time.Now()
		c.Set("lambdaStart", lambdaStart)
		c.Next()
		lambdaEnd := time.Now()

		c.Header("X-Lambda-Start-Time", lambdaStart.String())
		c.Header("X-Lambda-End-Time", lambdaEnd.String())
		c.Header("X-Lambda-Duration", lambdaEnd.Sub(lambdaStart).String())
	})

	r.POST("/math", func(c *gin.Context) {
		var payload struct {
			Numbers   []float64 `json:"numbers"`
			Operation string   `json:"operation"`
		}
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		var result float64
		if payload.Operation == "sum" {
			for _, num := range payload.Numbers {
				result += num
			}
		} else if payload.Operation == "product" {
			result = 1
			for _, num := range payload.Numbers {
				result *= num
			}
		} else {
			c.JSON(http.StatusBadRequest, gin.H{"error": "Unsupported operation"})
			return
		}

		c.JSON(http.StatusOK, gin.H{"result": result})
	})

	r.POST("/json", func(c *gin.Context) {
		var payload struct {
			Key   string `json:"key"`
			Value string `json:"value"`
		}
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		jsonData, _ := json.Marshal(map[string]string{payload.Key: payload.Value})
		c.JSON(http.StatusOK, gin.H{"json_data": string(jsonData)})
	})

	r.POST("/string", func(c *gin.Context) {
		var payload struct {
			Text    string `json:"text"`
			Pattern string `json:"pattern"`
		}
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		re := regexp.MustCompile(payload.Pattern)
		matches := re.FindAllString(payload.Text, -1)
		c.JSON(http.StatusOK, gin.H{"matches": matches})
	})

	r.POST("/compress", func(c *gin.Context) {
		var payload struct {
			Text string `json:"text"`
		}
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		var buf bytes.Buffer
		gzipWriter := gzip.NewWriter(&buf)
		_, _ = gzipWriter.Write([]byte(payload.Text))
		gzipWriter.Close()
		c.Data(http.StatusOK, "application/gzip", buf.Bytes())
	})

	r.POST("/image", func(c *gin.Context) {
		var payload struct {
			Text string `json:"text"`
		}
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
			return
		}

		// Dummy image creation simulation
		encodedImage := base64.StdEncoding.EncodeToString([]byte("fake_image_data"))
		c.JSON(http.StatusOK, gin.H{"image": encodedImage})
	})

	r.Run(":8080")
}
