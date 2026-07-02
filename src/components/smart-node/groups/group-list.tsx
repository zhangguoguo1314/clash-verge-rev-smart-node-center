import {
  Box,
  Chip,
  IconButton,
  Stack,
  Typography,
} from '@mui/material'
import {
  CheckCircleOutline,
  DeleteOutline,
  Edit,
} from '@mui/icons-material'
import { useTranslation } from 'react-i18next'

import type { SmartGroup } from '@/types/smart-node'

interface GroupListProps {
  groups: SmartGroup[]
  onEdit: (groupId: string) => void
  onDelete: (groupId: string) => void
  onDetect: (groupId: string) => void
  detecting: boolean
}

export const GroupList = ({
  groups,
  onEdit,
  onDelete,
  onDetect,
  detecting,
}: GroupListProps) => {
  const { t } = useTranslation('smartNode')

  if (groups.length === 0) {
    return (
      <Box
        sx={{
          py: 6,
          textAlign: 'center',
          color: 'text.secondary',
        }}
      >
        <Typography variant="body2">
          {t('groups.empty')}
        </Typography>
      </Box>
    )
  }

  return (
    <Stack spacing={1}>
      {groups.map((group) => (
        <Box
          key={group.id}
          sx={{
            p: 2,
            borderRadius: 2,
            border: 1,
            borderColor: 'divider',
            '&:hover': {
              bgcolor: 'action.hover',
            },
          }}
        >
          <Stack
            direction="row"
            alignItems="center"
            justifyContent="space-between"
          >
            <Box sx={{ minWidth: 0, flex: 1 }}>
              <Typography variant="subtitle2" fontWeight="bold">
                {group.name}
              </Typography>
              {group.description && (
                <Typography
                  variant="caption"
                  color="text.secondary"
                >
                  {group.description}
                </Typography>
              )}
              <Stack
                direction="row"
                spacing={0.5}
                mt={0.5}
                alignItems="center"
              >
                <Chip
                  label={`${group.node_uids.length} ${t('groups.nodes')}`}
                  size="small"
                  variant="outlined"
                />
                {group.auto_switch.enabled && (
                  <Chip
                    label={t('groups.autoSwitch')}
                    size="small"
                    color="primary"
                  />
                )}
              </Stack>
            </Box>
            <Stack direction="row" spacing={0.5}>
              <IconButton
                size="small"
                onClick={() => onDetect(group.id)}
                disabled={detecting}
              >
                <CheckCircleOutline fontSize="small" />
              </IconButton>
              <IconButton
                size="small"
                onClick={() => onEdit(group.id)}
              >
                <Edit fontSize="small" />
              </IconButton>
              <IconButton
                size="small"
                color="error"
                onClick={() => onDelete(group.id)}
              >
                <DeleteOutline fontSize="small" />
              </IconButton>
            </Stack>
          </Stack>
        </Box>
      ))}
    </Stack>
  )
}
