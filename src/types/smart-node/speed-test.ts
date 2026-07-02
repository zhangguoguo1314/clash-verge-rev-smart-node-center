export interface SpeedTestResult {
  uid: string
  tcp_latency?: number
  http_latency?: number
  https_latency?: number
  ttfb?: number
  download_speed_mbps?: number
  upload_speed_mbps?: number
  score: number
  error?: string
  test_time: string
}
