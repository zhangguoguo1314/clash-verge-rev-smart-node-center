import {
  Box,
  Button,
  Chip,
  Stack,
  Typography,
} from '@mui/material'
import { DeleteOutline } from '@mui/icons-material'
import { useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'

import { useHistory, usePool } from '@/hooks/use-smart-node'
import type { HistoryRecord } from '@/types/smart-node'

export const SmartNodeHistory = () => {
  const { t } = useTranslation('smartNode')
  const { pool } = usePool()
  const [selectedUid, setSelectedUid] = useState('')
  const [days, setDays] = useState(7)

  const {
    history,
    stats,
    cleanup,
    cleaning,
  } = useHistory(selectedUid, days)

  const nodeMap = useMemo(() => {
    const map = new Map<string, string>()
    for (const n of pool ?? []) {
      map.set(n.uid, n.name)
    }
    return map
  }, [pool])

  const getTypeColor = (type: HistoryRecord['type']) => {
    switch (type) {
      case 'speed_test':
        return 'primary'
      case 'health_check':
        return 'success'
      case 'switch':
        return 'warning'
      case 'group_detect':
        return 'info'
    }
  }

  return (
    <Stack spacing={2}>
      <Stack
        direction="row"
        alignItems="center"
        justifyContent="space-between"
      >
        <Typography variant="h6">
          {t('history.title')}
        </Typography>
        <Button
          size="small"
          variant="outlined"
          startIcon={<DeleteOutline />}
          onClick={cleanup}
          disabled={cleaning}
        >
          {t('history.cleanup')}
        </Button>
      </Stack>

      <Stack direction="row" spacing={1}>
        {[7, 14, 30].map((d) => (
          <Chip
            key={d}
            label={`${d}${t('history.days')}`}
            variant={days === d ? 'filled' : 'outlined'}
            onClick={() => setDays(d)}
            size="small"
          />
        ))}
      </Stack>

      {selectedUid && stats && (
        <Box
          sx={{
            display: 'grid',
            gridTemplateColumns:
              'repeat(auto-fit, minmax(140px, 1fr))',
            gap: 1,
          }}
        >
          <Stack p={1} borderRadius={1} bgcolor="action.hover">
            <Typography variant="caption" color="text.secondary">
              {t('history.successRate')}
            </Typography>
            <Typography variant="h6">
              {(stats.success_rate * 100).toFixed(1)}%
            </Typography>
          </Stack>
          <Stack p={1} borderRadius={1} bgcolor="action.hover">
            <Typography variant="caption" color="text.secondary">
              {t('history.avgLatency')}
            </Typography>
            <Typography variant="h6">
              {stats.avg_latency_ms.toFixed(0)} ms
            </Typography>
          </Stack>
          <Stack p={1} borderRadius={1} bgcolor="action.hover">
            <Typography variant="caption" color="text.secondary">
              {t('history.avgSpeed')}
            </Typography>
            <Typography variant="h6">
              {stats.avg_speed_mbps.toFixed(1)} Mbps
            </Typography>
          </Stack>
          <Stack p={1} borderRadius={1} bgcolor="action.hover">
            <Typography variant="caption" color="text.secondary">
              {t('history.onlineRate')}
            </Typography>
            <Typography variant="h6">
              {(stats.online_rate * 100).toFixed(1)}%
            </Typography>
          </Stack>
        </Box>
      )}

      <Stack spacing={0.5}>
        {history.map((record, index) => (
          <Stack
            key={`${record.timestamp}-${index}`}
            direction="row"
            alignItems="center"
            spacing={1}
            px={2}
            py={1}
            sx={{
              borderRadius: 1,
              '&:hover': {
                bgcolor: 'action.hover',
              },
            }}
          >
            <Chip
              label={record.type}
              color={getTypeColor(record.type)}
              size="small"
            />
            <Typography variant="body2" flex={1}>
              {nodeMap.get(record.node_uid) ?? record.node_uid}
            </Typography>
            <Typography
              variant="caption"
              color="text.secondary"
            >
              {record.timestamp}
            </Typography>
          </Stack>
        ))}
        {history.length === 0 && (
          <Typography
            variant="body2"
            color="text.secondary"
            textAlign="center"
            py={4}
          >
            {t('history.empty')}
          </Typography>
        )}
      </Stack>
    </Stack>
  )
}
