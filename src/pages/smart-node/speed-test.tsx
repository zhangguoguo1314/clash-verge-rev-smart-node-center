import {
  Button,
  LinearProgress,
  Stack,
  Typography,
} from '@mui/material'
import { PlayArrow, Stop } from '@mui/icons-material'
import { useMemo } from 'react'
import { useTranslation } from 'react-i18next'

import { usePool, useSpeedTest } from '@/hooks/use-smart-node'

export const SmartNodeSpeedTest = () => {
  const { t } = useTranslation('smartNode')
  const { pool } = usePool()
  const {
    results,
    testing,
    startTest,
    starting,
    stopTest,
    stopping,
  } = useSpeedTest()

  const handleStartAll = async () => {
    const uids = (pool ?? []).map((n) => n.uid)
    await startTest(uids, [
      'tcp',
      'http',
      'download',
    ])
  }

  const resultMap = useMemo(() => {
    const map = new Map<string, (typeof results)[number]>()
    for (const r of results ?? []) {
      map.set(r.uid, r)
    }
    return map
  }, [results])

  return (
    <Stack spacing={2}>
      <Stack
        direction="row"
        alignItems="center"
        justifyContent="space-between"
      >
        <Typography variant="h6">
          {t('speedTest.title')}
        </Typography>
        <Stack direction="row" spacing={1}>
          {testing ? (
            <Button
              size="small"
              variant="outlined"
              color="error"
              startIcon={<Stop />}
              onClick={stopTest}
              disabled={stopping}
            >
              {t('speedTest.stop')}
            </Button>
          ) : (
            <Button
              size="small"
              variant="contained"
              startIcon={<PlayArrow />}
              onClick={handleStartAll}
              disabled={starting}
            >
              {t('speedTest.startAll')}
            </Button>
          )}
        </Stack>
      </Stack>

      {testing && <LinearProgress />}
      <Stack spacing={0.5}>
        {(pool ?? []).map((node) => {
          const result = resultMap.get(node.uid)
          return (
            <Stack
              key={node.uid}
              direction="row"
              alignItems="center"
              justifyContent="space-between"
              px={2}
              py={1}
              sx={{
                borderRadius: 1,
                '&:hover': {
                  bgcolor: 'action.hover',
                },
              }}
            >
              <Typography variant="body2">
                {node.name}
              </Typography>
              <Stack
                direction="row"
                spacing={2}
                alignItems="center"
              >
                <Typography
                  variant="body2"
                  color="text.secondary"
                >
                  {result
                    ? `${result.tcp_latency ?? '-'} ms`
                    : '-'}
                </Typography>
                <Typography
                  variant="body2"
                  color="text.secondary"
                >
                  {result
                    ? `${result.download_speed_mbps?.toFixed(1) ?? '-'} Mbps`
                    : '-'}
                </Typography>
                <Typography
                  variant="body2"
                  fontWeight="bold"
                >
                  {result ? `Score: ${result.score}` : '-'}
                </Typography>
              </Stack>
            </Stack>
          )
        })}
      </Stack>
    </Stack>
  )
}
