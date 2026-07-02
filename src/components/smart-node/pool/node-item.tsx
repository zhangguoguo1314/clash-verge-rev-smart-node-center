import {
  Box,
  Chip,
  IconButton,
  Stack,
  Typography,
} from '@mui/material'
import { DeleteOutline } from '@mui/icons-material'
import { useTranslation } from 'react-i18next'

import { usePool } from '@/hooks/use-smart-node'

interface NodeItemProps {
  uid: string
  onDelete: (uid: string) => Promise<void>
  deleting: boolean
}

export const NodeItem = ({
  uid,
  onDelete,
  deleting,
}: NodeItemProps) => {
  const { t } = useTranslation('smartNode')
  const { pool } = usePool()

  const node = (pool ?? []).find((n) => n.uid === uid)
  if (!node) return null

  const statusColor =
    node.status === 'healthy'
      ? 'success'
      : node.status === 'unhealthy'
        ? 'error'
        : 'default'

  return (
    <Box
      sx={{
        p: 2,
        borderRadius: 2,
        border: 1,
        borderColor: 'divider',
      }}
    >
      <Stack
        direction="row"
        alignItems="center"
        justifyContent="space-between"
        mb={1}
      >
        <Typography variant="subtitle1" fontWeight="bold">
          {node.name}
        </Typography>
        <IconButton
          size="small"
          color="error"
          onClick={() => onDelete(uid)}
          disabled={deleting}
        >
          <DeleteOutline fontSize="small" />
        </IconButton>
      </Stack>

      <Stack spacing={0.5}>
        <Typography variant="body2" color="text.secondary">
          {node.protocol}://{node.address}:{node.port}
        </Typography>

        <Stack direction="row" spacing={1} alignItems="center">
          <Chip
            label={node.status}
            color={statusColor}
            size="small"
          />
          <Typography variant="caption" color="text.secondary">
            Score: {node.score}
          </Typography>
        </Stack>

        {node.tags.length > 0 && (
          <Stack direction="row" spacing={0.5} flexWrap="wrap" useFlexGap>
            {node.tags.map((tag) => (
              <Chip key={tag} label={tag} size="small" variant="outlined" />
            ))}
          </Stack>
        )}

        {node.latency_ms != null && (
          <Typography variant="caption" color="text.secondary">
            {t('pool.latency')}: {node.latency_ms} ms
          </Typography>
        )}
        {node.download_speed_mbps != null && (
          <Typography variant="caption" color="text.secondary">
            {t('pool.speed')}: {node.download_speed_mbps.toFixed(1)} Mbps
          </Typography>
        )}

        {node.source && (
          <Typography variant="caption" color="text.secondary">
            {t('pool.source')}: {node.source}
          </Typography>
        )}

        <Typography variant="caption" color="text.secondary">
          Fail: {node.fail_count}
        </Typography>
      </Stack>
    </Box>
  )
}
